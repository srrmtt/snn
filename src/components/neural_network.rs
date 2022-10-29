use std::vec;

use std::sync::mpsc::sync_channel;

use super::{input_layer::InputLayer, neural_layer::NeuralLayer, neuron::Neuron, output::OutputMonitor};

pub struct NeuralNetwork {
    input_layer: Option<InputLayer>,
    neural_layers: Vec<NeuralLayer>,
    output_monitor: Option<OutputMonitor>,
}

impl NeuralNetwork {
    pub fn new(
        v_threshold: f32,
        v_rest: f32,
        v_reset: f32,
        tao: f64,
        model: fn(i8, i8, f32, f32, f64, Vec<i32>) -> f32,
        npl: &[i8],
    ) -> Self {
        let mut layers = vec![];
        // npl parameter is 'neuoron per layer' the len of the array is the number of layers and the elements are the number of neurons for each one
        for (n_layer, n_neurons) in npl.iter().enumerate() {
            let mut nl = NeuralLayer::new(*n_neurons as usize);
            for i in 0..*n_neurons {
                nl.add_neuron(Neuron::new(
                    v_threshold,
                    v_rest,
                    v_reset,
                    tao,
                    model,
                    format!("l{}n{}", n_layer.to_string(), i.to_string()),
                ))
            }
            layers.push(nl);
        }

        Self {
            input_layer: None,
            neural_layers: layers,
            output_monitor: None,
        }
    }

    pub fn run(self) {
        let tid_input = match self.input_layer {
            None => panic!("Use connect inputs - Input layer not connected"),
            Some(il) => il.emit_spikes(),
        };
        let tid_output = match self.output_monitor {
            None => panic!("Use connect output - Output monitor not connected"),
            Some(om) => om.run(),
        };
        
        let mut v = vec![];
        for l in self.neural_layers {
            v.push(l.run_neurons());
        }


        
        let mut i = 0;

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

        tid_output.join();
    }

    pub fn connect(&mut self, from: usize, to: usize, weights: Vec<Vec<Option<i32>>>) {
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
            capacity = 10;
        }
        for (i, row) in weights.iter().enumerate() {
            // for each neuron connected to the sender add the receiver end
            for (j, weight) in row.iter().enumerate() {
                let (tx, rx) = sync_channel(capacity);
                match *weight {
                    None => continue,
                    Some(w) => {
                        println!("---[layer {}] connecting neuron {} with neuron {}",to, i, j);
                        self.neural_layers[to].add_synapse(j, w, rx);
                        // add the sender (tx) part of the channel to the 'to' layer
                        self.neural_layers[from].add_sender(i, tx);
                    }
                }
            }
        }
    }

    pub fn connect_inputs(&mut self, filenames: &[&str], weights: Vec<Vec<i32>>) {
        /*
         * Connette il layer di input con il primo layer (in posizione 0) della rete neurale. Questo metodo fallisce se non sono ancora stati
         * aggiunti dei layer alla rete oppure se ci sono problemi con la lettura del file.
         */
        if (&self.neural_layers).len() == 0 {
            panic!("Cannot link input with first layer, the layer does not exist.")
        }
        // crea il layer di input a partire dai file specificati
        self.input_layer = Some(InputLayer::from_files(filenames));

        // per ogni file
        for (i, row) in weights.iter().enumerate() {
            // sender: lato input layer
            // receiver: lato neuron layer
            for (j, weight) in row.iter().enumerate() {
                let (tx, rx) = sync_channel(0);
                self.input_layer.as_mut().unwrap().add_sender_to(i, tx);
                self.neural_layers[0].add_synapse(j, *weight, rx);
            }
        }
    }

    pub fn connect_output(&mut self, mut output_monitor: OutputMonitor){
        if self.neural_layers.len() == 0{
            panic!("add at least a layer before adding the output monitor");
        }
        let last_layer =  self.neural_layers.len() - 1 ;
        
        

        for neuron in self.neural_layers[last_layer].neurons.iter_mut(){
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
