use std::{
    sync::{Arc, Barrier},
    thread::{self, JoinHandle},
};

use std::sync::mpsc::{Receiver, Sender};

use crate::components::neuron::Neuron;

use super::{spike::Spike};
/*
Struttura contenitore di Neuroni
*/
pub struct NeuralLayer {
    pub neurons: Vec<Neuron>,
    // barriera che sincronizza len(neurons) neuroni.
    pub in_barrier: Option<Arc<Barrier>>,
    pub out_barrier: Option<Arc<Barrier>>,
    pub receiver: Option<Receiver<Vec<Spike>>>,
    pub sender: Option<Sender<Vec<Spike>>>,
    inhib_synapses_weights: Vec<Vec<f64>>,
    input_synapses_weights: Vec<Vec<f64>>,
    neurons_outputs: Vec<Spike>,
    last_fired: Vec<i8>,
        
}

impl NeuralLayer {
    pub fn new(n_neurons: usize, inhib_synapses_weights: Vec<Vec<f64>>, input_synapses_weights: Vec<Vec<f64>>) -> Self {
        println!("Creating a Neural Layer with {} neurons.", n_neurons);
        NeuralLayer {
            neurons: Vec::with_capacity(n_neurons),
            in_barrier: None,
            out_barrier: None,
            receiver: None, 
            sender: None,
            inhib_synapses_weights,
            input_synapses_weights,
            neurons_outputs: vec![],
            last_fired: vec![0; n_neurons],
        }
    }

    pub fn add_neuron(&mut self, neuron: Neuron) {
        // aggiunge un neurone al layer
        self.neurons.push(neuron);
    }
    fn check(&self) {
        if self.in_barrier.is_none() || self.out_barrier.is_none() {
            panic!("[Neural Layer]: call the connect before running the layer.\n\t\t---> Barrier is None");
        }
        if self.receiver.is_none(){
            panic!("[Neural Layer]: call the connect before running the layer.\n\t\t---> Receiver from the previous layer is None");
        }
        if self.sender.is_none() {
            panic!("[Neural Layer]: call the connect before running the layer.\n\t\t---> Sender to the next layer is None");
        }
    }
    // TODO: try to parallelize this layer
    pub fn run(mut self) -> JoinHandle<()>{
        // lancia n_neurons thread attraverso il metodo run() dei singoli neuroni
        // TODO: gestire gli errori
        self.check();
        

        let tid = thread::spawn(move || {
            // safe unwrap, check passed
            let receiver = self.receiver.unwrap();
            let sender = self.sender.unwrap();
            let mut spikes =vec![];
            loop {
                spikes = match receiver.recv(){
                    Ok(s) => s,
                    Err(e) => 
                        return,
                    
                };
                let barrier = self.in_barrier.as_ref().unwrap();
                //println!("[Neuron Layer]: Waiting at input barrier.");
                barrier.wait();
                let mut last_fired_new = vec![];
                for neuron in &mut self.neurons {
                    let curr_neuron_pos = neuron.position as usize; 
                    
                    // vettore di ingressi pesati provenienti dal layer precedente 
                    let mut weighted_inputs: Vec<f64> = spikes.iter().map(|&spike| {
                        let from = spike.n_neuron.unwrap() as usize;                             
                        if spike.output != 0{
                            return self.input_synapses_weights[curr_neuron_pos][from] as f64
                        }         
                        0.0           
                    }).collect();
                    let mut inhib_inputs: Vec<f64>= self.last_fired.iter().enumerate().map(|(from, spike)| {
                        if *spike != 0 {
                            return *spike as f64 * self.inhib_synapses_weights[curr_neuron_pos][from] as f64
                        }
                        0.0
                    }).collect();
                    weighted_inputs.append(&mut inhib_inputs);
                    let out = neuron.run(weighted_inputs);
                    last_fired_new.push(out);
                    //println!("{}: emitting {}", neuron.to_string(), out.output);
                }
                self.last_fired= last_fired_new.iter().map(|&spike| spike.output).collect::<Vec<i8>>().clone();
                sender.send(last_fired_new);
                let out_barrier = self.out_barrier.as_ref().unwrap();
                //println!("[Neuron Layer]: Waiting at out barrier.");
                out_barrier.wait();
            }
        });
        

        return tid;
    }
    pub fn set_barrier(&mut self, barrier: Arc<Barrier>,is_input: bool){
        if is_input {
            self.in_barrier = Some(barrier);
        }else{
            self.out_barrier = Some(barrier);
        }
    }
    pub fn set_sender(&mut self, sender: Sender<Vec<Spike>>){
        self.sender = Some(sender)
    }

    pub fn set_receiver(&mut self, receiver: Receiver<Vec<Spike>>){
        self.receiver = Some(receiver)
    }

    pub fn to_string(&self) -> String{
        format!("with [{}] neurons.", self.neurons.len())
    }
}