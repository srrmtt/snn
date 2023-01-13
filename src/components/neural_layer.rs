use std::{
    sync::{Arc, Barrier},
    thread::{self, JoinHandle},
};

use std::sync::mpsc::{Receiver, Sender};

use crate::components::neuron::Neuron;

use super::{synapse::Synapse, spike::Spike, errors::SNNError};
/*
Struttura contenitore di Neuroni
*/
pub struct NeuralLayer {
    pub neurons: Vec<Neuron>,
    // barriera che sincronizza len(neurons) neuroni.
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
        // aggiunge un neurone al layer
        self.neurons.push(neuron);
    }

    pub fn run_neurons(self) -> Vec<JoinHandle<()>> {
        // lancia n_neurons thread attraverso il metodo run() dei singoli neuroni
        // TODO: gestire gli errori
        let mut tids = vec![];
        for mut neuron in self.neurons {
            // clone della barrier per condividerla con i thread da sincronizzare
            let barrier = Arc::clone(&self.barrier);

            let tid = thread::spawn(move || {
                let res = neuron.run(barrier);
                if res.is_err(){
                    panic!("[Neural Layer]: {:?}", res)
                }
            });
            tids.push(tid);
        }

        return tids;
    }

    pub fn add_synapse(&mut self, neuron: usize, weight: f64, channel: Receiver<Spike>)  -> Result<(), SNNError>{
        // aggiunge una sinapsi ricevendo peso e receiver a un neurone, return di result se neuron è out of bounds
        let s = Synapse::new(weight, channel);
        let len = self.neurons.len();
        if neuron >= self.neurons.len(){
            return Err(SNNError::OutOfIndexError(format!("Trying to add synapses to neuron [{neuron}] but there are only {len} in the layer")));
        }
        self.neurons[neuron].synapses.push(s);
        Ok(())
    }

    pub fn add_sender(&mut self, neuron: usize, channel: Sender<Spike>) {
        // aggiunge un sender al neuron-esimo neurone, restituire un error se è out of bounds 
        self.neurons[neuron].output.push(channel);
    }
}
