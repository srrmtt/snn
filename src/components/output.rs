use super::spike::Spike;
use std::{
    sync::mpsc::{Receiver},
    thread::{self, JoinHandle},
};

use super::errors::SNNError;

/*
Terminale che si può connettere a un layer per osservarne gli output.
*/

pub struct OutputMonitor {
    // connessioni in ingresso
    receivers: Vec<Receiver<Spike>>,
    // outputs è il vettore che colleziona il numero di spike a 1 per ogni neurone dell'ultimo layer 
    // corrispondenti a spike.n_neuron. 
    outputs: Vec<i32>,
    // tempo locale al monitor
    ts: i32,
}

impl OutputMonitor {
    pub fn new(n_lastlayer: usize) -> Self {
        // costruttore
        Self {
            receivers: vec![],
            outputs: vec![0; n_lastlayer],
            ts: 0,
        }
    }

    pub fn add_receiver(&mut self, receiver: Receiver<Spike>) {
        // aggiunge un ricevitore in ingresso
        self.receivers.push(receiver);
    }

    pub fn receive(&mut self) -> Result<() , SNNError> {
        // riceve gli impulsi dal layer precedente, se va a buon fine restituisce il vettore di impulsi letti, altrimenti un RecvError

        // per ogni ricevitore
        for receiver in &self.receivers {
            // riceve gli impulsi
            let out = receiver.recv();
            match out {
                Ok(spike) => {
                    let n_neuron;
                    match spike.n_neuron {
                        Some(index) => n_neuron = index as usize,
                        None => {
                            return Err(SNNError::InconnectedOutput("Connect the last layer with the output layer before calling run".to_string()));
                        }
                    };
                    // aggiorna il vettore in posizione n_neuron con la spike ricevuta (+0 o +1)
                    self.outputs[n_neuron] += spike.output as i32;
                }
                Err(_) => return Err(SNNError::EmptyChannelError("Comunication ended".to_string())),
            }
        }

        Ok(())
    }

    pub fn run(mut self) -> JoinHandle<Vec<i32>> {
        // lancia un thread e restituisce un Join Handle, cambiare il return in Result e sostituire il break con un return di RecvError
        thread::spawn(move || {
            loop {
                let res = self.receive();
                match res {
                    Ok(_) => {
                        self.ts += 1;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
            self.outputs
        })
    }
}
