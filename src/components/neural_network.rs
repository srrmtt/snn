use crossbeam::channel::unbounded;

use super::{input_layer::InputLayer, neural_layer::NeuralLayer, neuron::Neuron};

pub struct NeuralNetwork {
    // neural network parameters
    v_threshold: f32,
    v_rest: f32,
    v_reset: f32,
    // LIR model function signature, maybe to be generalized
    model: Box<dyn Fn(i8, i8, f32, f32, f64, Vec<i32>) -> f32>,

    input_reader: InputLayer,

    layers: Vec<NeuralLayer>,
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

        for n_neurons in npl {
            let mut nl = NeuralLayer::new();
            for _ in 0..*n_neurons {
                nl.add_neuron(Neuron::new(v_threshold, v_rest, v_reset, tao, model))
            }
            layers.push(nl);
        }

        Self {
            v_threshold,
            v_rest,
            v_reset,
            model: Box::new(model),
            input_reader: InputLayer::empty_reader(),
            layers,
        }
    }

    pub fn run(self, output_file: &str) {
        let tid_input = self.input_reader.emit_spikes();

        tid_input.join();
        println!("printing the results in {}", output_file);
    }

    pub fn connect(&mut self, l1: i8, l2: i8, weights: Vec<Vec<i32>>) {
        return;
    }

    pub fn connect_inputs(&mut self, filenames: &[&str], weights: Vec<Vec<i32>>) {
        if self.layers.len() == 0 {
            panic!("Cannot link input with first layer, if the layer does not exist.")
        }
        for filename in filenames {
            let v = InputLayer::read_file(filename);
            match v {
                Ok(content) => self.input_reader.inputs.push(content),
                Err(e) => panic!("Error: {:?}", e),
            }
        }
        let (tx, rx) = unbounded::<Vec<i8>>();
        self.input_reader.out = Option::Some(tx);
        let mut i = 0;
        for neuron in &mut self.layers[0].neurons {
            neuron.exitatory_weights = weights[i].clone();
            neuron.synapses = Some(rx.clone());
            i += 1;
        }
    }
}
