
use crate::components::neuron::Neuron;

pub struct NeuralLayer {
    pub neurons: Vec<Neuron>,
}

impl NeuralLayer {
    pub fn new() -> Self {
        NeuralLayer { neurons: vec![] }
    }

    pub fn add_neurons(&mut self, new_neurons: &mut Vec<Neuron>) {
        self.neurons.append(new_neurons);
    }

    pub fn add_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron);
    }
}
