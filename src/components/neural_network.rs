use libm::exp;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Barrier};
use std::{sync::mpsc::channel, vec};

use super::{
    input_layer::InputLayer,
    neural_layer::NeuralLayer,
    neuron::Neuron,
    output::OutputMonitor,
    spike::Spike,
};

fn lif(ts: i32, ts_1: i32, v_rest: f64, v_mem_old: f64, tao: f64, weights: Vec<f64>) -> f64 {
    let k = -((ts - ts_1) as f64 / tao);

    let exponential = exp(k);

    let v_mem = v_rest + (v_mem_old - v_rest) * exponential;

    let weight = weights.iter().sum::<f64>();
    return v_mem + weight;
}

#[derive(Debug, Deserialize)]
struct Value {
    thresholds: Vec<Vec<f64>>,
    rest_potential: f64,
    reset_potential: f64,
    tau: f64,
    intra_layer_weights: Vec<Vec<Vec<f64>>>,
    input_weights: Vec<Vec<Vec<f64>>>,
    inputs: String,
}

/*
Classe contenitore dei vari layer, attraverso i vari metodi connect si possono aggiungere le varie componenti e collegarle tra loro.
Attraverso il metodo run() si lancia la simulazione.
*/
pub struct NeuralNetwork {
    input_layer: Option<InputLayer>,
    neural_layers: Vec<NeuralLayer>,
    output_monitor: Option<OutputMonitor>,
}

impl NeuralNetwork {
    pub fn new(
        // costruttore
        v_rest: f64,
        v_reset: f64,
        tao: f64,
        model: fn(i32, i32, f64, f64, f64, Vec<f64>) -> f64,
        thresholds: Vec<Vec<f64>>,
        intra_layer_weights: Vec<Vec<Vec<f64>>>,
        input_layer_weights: Vec<Vec<Vec<f64>>>,
    ) -> Self {
        let mut layers = vec![];
        for (n_layer, layer) in thresholds.iter().enumerate() {
            let mut nl = NeuralLayer::new(
                layer.len() - 1,
                intra_layer_weights[n_layer].clone(),
                input_layer_weights[n_layer].clone(),
            );
            let mut n_neuron: i32 = 0;
            for threshold in layer {
                nl.add_neuron(Neuron::new(
                    *threshold, v_rest, v_reset, tao, model, n_neuron,
                ));
                n_neuron += 1;
            }
            layers.push(nl);
        }

        Self {
            input_layer: None,
            neural_layers: layers,
            output_monitor: None,
        }
    }

    pub fn from_json(path: &str) -> NeuralNetwork {
        let file = File::open(path).unwrap();
        let parameters: Value = serde_json::from_reader(file).expect("JSON was not well-formatted");
        let last_layer_len = parameters.thresholds.last().unwrap().len();
        let mut nn = NeuralNetwork::new(
            parameters.rest_potential,
            parameters.reset_potential,
            parameters.tau,
            lif,
            parameters.thresholds,
            parameters.intra_layer_weights,
            parameters.input_weights,
        );
        let input_layer_res = InputLayer::from_file(&parameters.inputs, '\n');
        let input_layer = match input_layer_res {
            Ok(il) => il,
            Err(e) => panic!("{:?}", e),
        };
        nn.connect_input_layer(input_layer);
        for i in 0..nn.neural_layers.len() - 1 {
            nn.connect(i, i + 1);
        }
        let om = OutputMonitor::new(last_layer_len);
        nn.connect_output(om);
        return nn;
    }

    pub fn run(self, output_file: &str) {
        // avvia tutti gli input layer e colleziona gli handler per fare join in caso di successo, None altrimenti (cambiare Option in Result)
        let tid_input_layer = match self.input_layer {
            None => panic!("Use connect output - Output monitor not connected"),
            Some(il) => il.run(),
        };

        // lancia il metodo che riceve le spike di output dell'ultimo layer, panic se non è connesso ( cambiare option in Result )
        let tid_output = match self.output_monitor {
            None => panic!("Use connect output - Output monitor not connected"),
            Some(om) => om.run(),
        };

        // lancia tutti i neuroni di ogni layer, cambiare il metodo run neurons in modo che restituisca un Result<Error, Ok(Vec<Handle>)
        let mut tids_nl = vec![];
        for layer in self.neural_layers {
            let tid_nl = layer.run();
            tids_nl.push(tid_nl);
        }

        let r = tid_input_layer.join();
        match r {
            Ok(_) => println!("\t\t- input layer thread: OK"),
            Err(e) => println!("error {:?} during join neural_netowork run() method", e),
        }

        for tid in tids_nl {
            let r = tid.join();
            match r {
                Ok(_) => println!("\t\t- neuron thread: OK"),
                Err(e) => println!("error {:?} during join neural_netowork run() method", e),
            }
        }

        // TODO: handle join
        let result = tid_output.join();
        match result {
            Ok(counted_output) => print(counted_output, output_file),
            Err(e) => panic!("{:?}", e),
        };
    }

    pub fn connect(&mut self, from: usize, to: usize) {
        /*
         * Questo metodo connette il layer from con il layer to, se i valori coincidono significa che si stanno collegando neuroni dello stesso layer
         * e quindi si stano creando sinapsi inibitorie. In generale si utilizza una matrice di pesi in il primo indice indica il neurone del layer a
         * 'sinistra' (from), mentre il secondo quello a destra (to), per specificare due neuroni non collegati utilizzare *None*. Nel caso di sinapsi
         * inibitorie utilizzare una matrice quadrata con diagonale pari a None.
         */
        let barrier = Arc::new(Barrier::new(2));
        let n_layers = self.neural_layers.len();
        // check if the two parameters are conform with the net's dimension
        if from >= n_layers || to >= n_layers {
            panic!(
                "Cannot link the layer {} with the {} one, the net has only {} layers",
                from, to, n_layers
            );
        }
        //TODO  check the weights dimension
        let c = Arc::clone(&barrier);

        let (tx, rx) = channel::<Vec<Spike>>();
        self.neural_layers[to].set_receiver(rx);
        self.neural_layers[to].set_barrier(c, true);

        let c = Arc::clone(&barrier);
        self.neural_layers[from].set_sender(tx);
        self.neural_layers[from].set_barrier(c, false);
    }

    pub fn connect_input_layer(&mut self, mut input_layer: InputLayer) {
        /*
         * Connette il layer di input con il primo layer (in posizione 0) della rete neurale. Questo metodo fallisce se non sono ancora stati
         * aggiunti dei layer alla rete oppure se ci sono problemi con la lettura del file.
         */
        if (&self.neural_layers).len() == 0 {
            panic!("Cannot link input with first layer, the layer does not exist.")
        }
        let barrier = Arc::new(Barrier::new(2));
        let (tx, rx) = channel::<Vec<Spike>>();
        let c = Arc::clone(&barrier);

        input_layer.set_sender(tx);
        input_layer.set_barrier(c);

        self.neural_layers[0].set_receiver(rx);

        let c = Arc::clone(&barrier);
        self.neural_layers[0].set_barrier(c, true);
        self.input_layer = Some(input_layer);
    }

    // TODO: return un errore al posto del panic, OK(()) se tutto funziona
    pub fn connect_output(&mut self, mut output_monitor: OutputMonitor) {
        // Connette l'ultimo layer con un output monitor, consuma l'ouput monitor e lo assegna alla rete.
        let barrier = Arc::new(Barrier::new(2));
        // controllo che esista almeno un layer
        if self.neural_layers.len() == 0 {
            panic!("add at least a layer before adding the output monitor");
        }
        // calcolo index ultimo layer
        let (tx, rx) = channel::<Vec<Spike>>();
        let c = Arc::clone(&barrier);
        let index = self.neural_layers.len() - 1;

        let last_layer = &mut self.neural_layers[index];
        last_layer.set_barrier(c, false);
        last_layer.set_sender(tx);

        let c = Arc::clone(&barrier);
        output_monitor.set_receiver(rx);
        output_monitor.set_barrier(c);
        self.output_monitor = Some(output_monitor);
    }

    pub fn to_string(&self) -> String {
        format!(
            "Neural Network with:\n\t- {}\n\t- [{}] neural layers",
            self.input_layer.as_ref().unwrap().to_string(),
            self.neural_layers.len()
        )
    }
}

pub fn print(result: Vec<i32>, path: &str) {
    for i in &result {
        println!("{}", i);
    }
    let mut output_file = File::create(path).unwrap();
    for o in result {
        write!(output_file, "{}\n", o.to_string()).expect("Unable to write output length");
    }
}
