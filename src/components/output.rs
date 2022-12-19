use super::spike::Spike;
use std::{
    sync::{mpsc::{Receiver, RecvError}, Barrier, Arc},
    thread::{self, JoinHandle},
};

/*
Terminale che si può connettere a un layer per osservarne gli output.
*/

pub struct OutputMonitor {
    // connessioni in ingresso
    receiver: Option<Receiver<Vec<Spike>>>,
    // output proveniente dal layer precedente
    // TODO sostituire i8 con la classe Spike da creare, in questo modo possiamo conoscere il
    // neurone di provenienza e ordinare gli output
    outputs: Vec<i32>,
    barrier: Option<Arc<Barrier>>,
    ts: i32,
}

impl OutputMonitor {
    pub fn new(n_lastlayer: usize) -> Self {
        // costruttore
        Self {
            receiver: None,
            outputs: vec![0; n_lastlayer],
            ts: 0,
            barrier: None
        }
    }

    pub fn set_receiver(&mut self, receiver: Receiver<Vec<Spike>>) {
        // aggiunge un ricevitore in ingresso
        self.receiver = Some(receiver);
    }

    pub fn set_barrier(&mut self, barrier: Arc<Barrier>){
        self.barrier = Some(barrier);
    }

    fn receive(&mut self) -> Result<Vec<Spike>, RecvError> {
       
        // per ogni ricevitore
        let receiver = self.receiver.as_ref().unwrap();
        match receiver.recv() {
            Ok(spike) => Ok(spike),
            Err(e) => return Err(e),
        }
    }

    pub fn run(mut self) -> JoinHandle<Vec<i32>> {
        // lancia un thread e restituisce un Join Handle, cambiare il return in Result e sostituire il break con un return di RecvError
        if self.receiver.is_none() {
            panic!("[Output Monitor]: ERROR connect this module with a layer: receiver is None.")
        }
        if self.barrier.is_none() {
            panic!("[Output Monitor]: ERROR connect this module with a layer: barrier is None.")
        }
        thread::spawn(move || {
            loop {
                let res = self.receive();
                let barrier = self.barrier.as_ref().unwrap();
                barrier.wait();
                match res {
                    Ok(outs) => {
                        //println!("\t Output Monitor: {} at [{}]", outs.into_iter().sum::<i8>(), self.ts);
                        for spike in outs{
                            let n_neuron = spike.n_neuron.unwrap();
                            self.outputs[n_neuron as usize] += spike.output as i32;
                        }
                        self.ts += 1;
                        if self.ts == 3500{
                            break;
                        }
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
