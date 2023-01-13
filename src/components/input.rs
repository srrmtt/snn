use std::sync::{Arc, Barrier};
use std::vec;

use std::sync::mpsc::Sender;

use super::errors::SNNError;
use super::spike::Spike;

/*
The input class contiains the logic to emit single spike to the first neural layer.
*/
pub struct Input {
    // vettore di spike che invia: a ogni posizione corrisponde un ts e la cella corrisponde alla spike da inviare   
    spikes: Vec<i8>,
    // vettori di sender collegati a ogni neurone del primo layer
    senders: Vec<Sender<Spike>>,
}

impl Input {
    pub fn new(spikes: Vec<i8>) -> Self {
        // costruttutore
        Self {
            spikes,
            senders: vec![],
        }
    }

    fn emit(&self, spike: Spike) -> Result<(), SNNError>{
        // emette una spike sul Sender, invia una spike ai neuroni collegati
        for input in &self.senders {
            // TODO handle SendError
            let r = input.send(spike);
            match r {
                Ok(()) => continue,
                Err(_) => return Err(SNNError::InconnectedInput("[Input] Connect this input with a neuron before calling the emit method.".to_string())),
            }
        }
        Ok(())
    }
    pub fn run(self, barrier: Arc<Barrier>) {
        // logic of the whole input emit spike until the input vector is empty
        for spike in &self.spikes {
            // handle the return result
            match self.emit(Spike::new(*spike, None)){
                Ok(_) => {},
                Err(e) => panic!("{:?}", e)
            }
            barrier.wait();
        }
    }
    pub fn is_empty_sender(&self) -> bool {
        return self.senders.is_empty();
    }

    pub fn add_sender(&mut self, tx: Sender<Spike>) {
        self.senders.push(tx);
    }
}
