use std::{
    sync::{Arc, Barrier},
    thread::{self, JoinHandle},
};

use std::sync::mpsc::{Receiver, SyncSender};

use crate::components::neuron::Neuron;

use super::synapse::Synapse;

pub struct NeuralLayer {
    pub neurons: Vec<Neuron>,
    pub barrier: Arc<Barrier>,
}

impl NeuralLayer {
    pub fn new(n_neurons: usize) -> Self {
        println!("Creating a Neural Layer with {} neurons.", n_neurons);
        NeuralLayer {
            neurons: Vec::with_capacity(n_neurons),
            barrier: Arc::new(Barrier::new(n_neurons)),
        }
    }

    pub fn add_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron);
    }

    pub fn run_neurons(self) -> Vec<JoinHandle<()>> {
        let mut tids = vec![];
        for mut neuron in self.neurons {
            let barrier = Arc::clone(&self.barrier);
            let tid = thread::spawn(move || {
                neuron.start(barrier);
            });
            tids.push(tid);
        }

        return tids;
    }

    pub fn add_synapse(&mut self, neuron: usize, weight: i32, channel: Receiver<i8>) {
        let s = Synapse::new(weight, channel);
        println!("adding synapses to neuron [{}]", &neuron);
        self.neurons[neuron].synapses.push(s);
    }

    pub fn add_sender(&mut self, neuron: usize, channel: SyncSender<i8>) {
        self.neurons[neuron].output.push(channel);
    }
}
