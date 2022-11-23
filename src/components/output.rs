use super::{spike::Spike, neural_network::print};
use std::{
    sync::mpsc::{Receiver, RecvError},
    thread::{self, JoinHandle},
};

/*
Terminale che si può connettere a un layer per osservarne gli output.
*/

pub struct OutputMonitor {
    // connessioni in ingresso
    receivers: Vec<Receiver<Spike>>,
    // output proveniente dal layer precedente
    // TODO sostituire i8 con la classe Spike da creare, in questo modo possiamo conoscere il
    // neurone di provenienza e ordinare gli output
    outputs: Vec<i32>,
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

    pub fn receive(&mut self) -> Result<Vec<Spike>, RecvError> {
        // riceve gli impulsi dal layer precedente, se va a buon fine restituisce il vettore di impulsi letti, altrimenti un RecvError

        // vettore di impulsi in ingresso
        let mut outs = vec![];

        // per ogni ricevitore
        for receiver in &self.receivers {
            // riceve gli impulsi
            let out = receiver.recv();
            match out {
                Ok(spike) => {
                    let n_neuron;
                    match spike.n_neuron {
                        Some(index) => n_neuron = index as usize,
                        None => panic!("Cannot connect input layer with output monitor"),
                    };
                    self.outputs[n_neuron] += spike.output as i32;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(outs)
    }

    pub fn run(mut self) -> JoinHandle<(Vec<i32>)> {
        // lancia un thread e restituisce un Join Handle, cambiare il return in Result e sostituire il break con un return di RecvError
        thread::spawn(move || {
            loop {
                let res = self.receive();
                match res {
                    Ok(outs) => {
                        //   println!("\t Output Monitor: {} at [{}]", outs.into_iter().sum::<i8>(), self.ts);
                        println!("{}",self.ts);
                        self.ts += 1;
                    }
                    Err(e) => break
                }
            }
            self.outputs
        })
    }
}
