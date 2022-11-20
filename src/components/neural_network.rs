use std::vec;

use std::sync::mpsc::sync_channel;

use libm::exp;
use std::fs::File;
use serde::Deserialize;

use super::{input_layer::InputLayer, neural_layer::NeuralLayer, neuron::Neuron, output::OutputMonitor};

fn lif(ts: i8, ts_1: i8, v_rest: f32, v_mem_old: f32, tao: f64, weights: Vec<f32>) -> f32 {
    let k = -(ts - ts_1) as f64 / tao;

    let exponential = std::f64::consts::E.powf(k) as f32;

    let v_mem = v_rest + (v_mem_old - v_rest) * exponential;

    let weight = weights.iter().fold(0.0, |sum, x| sum + x) as f32;
    return v_mem + weight;
}

#[derive(Debug, Deserialize)]
struct Value {
    thresholds: Vec<Vec<f32>>,
    rest_potential: f32,
    reset_potential: f32,
    tau: f64,
    intra_layer_weights: Vec<Vec<Vec<f32>>>,
    input_weights: Vec<Vec<Vec<f32>>>,
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
        v_rest: f32,
        v_reset: f32,
        tao: f64,
        model: fn(i8, i8, f32, f32, f64, Vec<f32>) -> f32,
        thresholds: Vec<Vec<f32>>,
    ) -> Self {
        let mut layers = vec![];
        let mut n_layer: i32 = 0;
        for layer in thresholds {
            let mut nl = NeuralLayer::new(layer.len() as usize);
            let mut n_neuron: i32 = 0;
            for threshold in layer {
                nl.add_neuron(Neuron::new(
                    threshold,
                    v_rest,
                    v_reset,
                    tao,
                    model,
                    format!("l{}n{}", n_layer.to_string(), n_neuron.to_string()),
                ));
                n_neuron+=1;
            }
            layers.push(nl);
            n_layer+=1;
        } 

        Self {
            input_layer: None,
            neural_layers: layers,
            output_monitor: None,
        }
    }


    pub fn from_JSON(path: &str)->NeuralNetwork{
        let file = File::open(path).unwrap();
        let parameters: Value = serde_json::from_reader(file).expect("JSON was not well-formatted");
        let mut nn = NeuralNetwork::new(parameters.rest_potential, parameters.reset_potential, parameters.tau, lif, parameters.thresholds);        
        for i in 0..nn.neural_layers.len() {
            nn.connect(i,i,parameters.intra_layer_weights[i].clone());
        }
        for i in 0..nn.neural_layers.len()-1 {
            nn.connect(i,i+1,parameters.input_weights[i+1].clone());
        }

        nn.connect_inputs(&parameters.inputs,parameters.input_weights[0].clone());
        return nn;   
    }   

    pub fn run(self) {
        // lancia la simulazione di tutta la rete neurale, wrapper di tutti i metodi di run 
        
        // avvia tutti gli input layer e colleziona gli handler per fare join in caso di successo, None altrimenti (cambiare Option in Result)
        let tid_input = match self.input_layer {
            None => panic!("Use connect inputs - Input layer not connected"),
            Some(il) => il.emit_spikes(),
        };
        
        // lancia il metodo che riceve le spike di output dell'ultimo layer, panic se non è connesso ( cambiare option in Result ) 
        let tid_output = match self.output_monitor {
            None => panic!("Use connect output - Output monitor not connected"),
            Some(om) => om.run(),
        };
        
        // lancia tutti i neuroni di ogni layer, cambiare il metodo run neurons in modo che restituisca un Result<Error, Ok(Vec<Handle>)
        let mut v = vec![];
        for l in self.neural_layers {
            v.push(l.run_neurons());
        }


        
        let mut i = 0;
        // join dei vari thread
        for tid in tid_input {
            let r = tid.join();
            match r {
                Ok(_) => println!("\t\t- input thread[{}]: OK", i),
                Err(e) => println!("error {:?} during join neural_netowork run() method", e),
            }
            i += 1;
        }

        for tids in v {
            for tid in tids {
                let r = tid.join();
                match r {
                    Ok(_) => println!("\t\t- neuron thread: OK"),
                    Err(e) => println!("error {:?} during join neural_netowork run() method", e),
                }
            }
        }
        // TODO: handle join 
        tid_output.join();
    }

    pub fn connect(&mut self, from: usize, to: usize, weights: Vec<Vec<f32>>) {
        /*
         * Questo metodo connette il layer from con il layer to, se i valori coincidono significa che si stanno collegando neuroni dello stesso layer
         * e quindi si stano creando sinapsi inibitorie. In generale si utilizza una matrice di pesi in il primo indice indica il neurone del layer a
         * 'sinistra' (from), mentre il secondo quello a destra (to), per specificare due neuroni non collegati utilizzare *None*. Nel caso di sinapsi
         * inibitorie utilizzare una matrice quadrata con diagonale pari a None.
         */
        let n_layers = self.neural_layers.len();
        // check if the two parameters are conform with the net's dimension
        if from >= n_layers || to >= n_layers {
            panic!(
                "Cannot link the layer {} with the {} one, the net has only {} layers",
                from, to, n_layers
            );
        }
        //TODO  check the weights dimension
        let mut capacity = 0;
        if to == from {
            capacity = 1;
        }
        for (i, row) in weights.iter().enumerate() {
            // for each neuron connected to the sender add the receiver end
            for (j, weight) in row.iter().enumerate() {
                let (tx, rx) = sync_channel(capacity);
                if *weight != 0.0 {
                        // println!("---[layer {} - {}] connecting neuron {} with neuron {}",to, from, i, j);
                        self.neural_layers[to].add_synapse(j, *weight, rx);
                        // add the sender (tx) part of the channel to the 'to' layer
                        self.neural_layers[from].add_sender(i, tx);
                    }
            }
        }
    }

    pub fn connect_inputs(&mut self, filename: &str, weights: Vec<Vec<f32>>) {
        /*
         * Connette il layer di input con il primo layer (in posizione 0) della rete neurale. Questo metodo fallisce se non sono ancora stati
         * aggiunti dei layer alla rete oppure se ci sono problemi con la lettura del file.
         */
        if (&self.neural_layers).len() == 0 {
            panic!("Cannot link input with first layer, the layer does not exist.")
        }
        
        // crea il layer di input a partire dai file specificati
        //  self.input_layer = Some(InputLayer::from_files(filenames));
        self.input_layer = Some(InputLayer::from_file(filename,'\n').unwrap());
        // per ogni file
        
        for (i, row) in weights.iter().enumerate() {
            // sender: lato input layer
            // receiver: lato neuron layer
            for (j, weight) in row.iter().enumerate() {
                let (tx, rx) = sync_channel(0);
                self.input_layer.as_mut().unwrap().add_sender_to(j, tx);
                self.neural_layers[0].add_synapse(i, *weight, rx);
            }
        }
    }

    // TODO: return un errore al posto del panic, OK(()) se tutto funziona 
    pub fn connect_output(&mut self, mut output_monitor: OutputMonitor){
        // Connette l'ultimo layer con un output monitor, consuma l'ouput monitor e lo assegna alla rete. 
        
        // controllo che esista almeno un layer 
        if self.neural_layers.len() == 0{
            panic!("add at least a layer before adding the output monitor");
        }
        // calcolo index ultimo layer
        let last_layer =  self.neural_layers.len() - 1 ;

        
        for neuron in self.neural_layers[last_layer].neurons.iter_mut(){
            // assegna ad ogni neurone l'estremità di sender e aggiunge all'output monitor i receiver
            let (tx, rx) = sync_channel::<i8>(0);    
            neuron.output.push(tx.clone());
            output_monitor.add_receiver(rx);
        }

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
