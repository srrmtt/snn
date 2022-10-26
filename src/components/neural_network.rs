use std::vec;

use crossbeam::channel::bounded;

use super::{input_layer::InputLayer, neural_layer::NeuralLayer, neuron::Neuron};

pub struct NeuralNetwork {
    input_layer: Option<InputLayer>,
    neural_layers: Vec<NeuralLayer>,
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
        for n_neurons in npl {
            let mut nl = NeuralLayer::new(*n_neurons as usize);
            for _ in 0..*n_neurons {
                nl.add_neuron(Neuron::new(v_threshold, v_rest, v_reset, tao, model))
            }
            layers.push(nl);
        }

        Self {
            input_layer: None,
            neural_layers: layers,
        }
    }

    pub fn run(self, output_file: &str) {
        let tid_input = match self.input_layer {
            None => panic!("Use connect inputs - Input layer not connected"),
            Some(il) => il.emit_spikes(),
        };
        let mut v = vec![];
        for l in self.neural_layers {
            v.push(l.run_neurons());
        }

        for tid in tid_input {
            tid.join();
        }
        for tids in v {
            for tid in tids {
                tid.join();
            }
        }
        println!("printing the results in {}", output_file);
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

        // if the layer wants to connect to itself it means that the synapses are inhibitory
        let inhibitory = from == to;

        for (i, row) in weights.iter().enumerate() {
            let (tx, rx) = bounded::<i8>(0);
            // add the sender (tx) part of the channel to the 'to' layer
            self.neural_layers[from].add_sender(i, tx);
            // for each neuron connected to the sender add the receiver end
            for (j, weight) in row.iter().enumerate() {
                match *weight {
                    None => continue,
                    Some(w) => {
                        self.neural_layers[to].add_synapse(j, w, rx.clone());
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

        // collegamento dei due layer
        for (i, row) in weights.iter().enumerate() {
            // sender: lato input layer
            // receiver: lato neuron layer
            let (tx, rx) = bounded::<i8>(0);
            self.input_layer.as_mut().unwrap().set_input_sender(i, tx);

            for (j, weight) in row.iter().enumerate() {
                self.neural_layers[0].add_synapse(j, *weight, rx.clone());
            }
        }
    }
}
