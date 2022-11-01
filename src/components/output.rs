use std::{sync::mpsc::{Receiver, RecvError}, thread::{JoinHandle, self}};

/*
Terminale che si può connettere a un layer per osservarne gli output.
*/

pub struct OutputMonitor {
    // connessioni in ingresso
    receivers: Vec<Receiver<i8>>,
    // output file
    filename: String,
    // output proveniente dal layer precedente
    // TODO sostituire i8 con la classe Spike da creare, in questo modo possiamo conoscere il  
    // neurone di provenienza e ordinare gli output
    outputs: Vec<i8>,
}

impl OutputMonitor {
    pub fn new(filename: &str) -> Self {
        // costruttore
        Self {
            receivers: vec![],
            outputs: vec![],
            filename: filename.to_string(),
        }
    }

    pub fn add_receiver(&mut self, receiver: Receiver<i8>) {
        // aggiunge un ricevitore in ingresso
        self.receivers.push(receiver);
    }

    pub fn receive(&self) -> Result<Vec<i8>, RecvError>{
        // riceve gli impulsi dal layer precedente, se va a buon fine restituisce il vettore di impulsi letti, altrimenti un RecvError

        // vettore di impulsi in ingresso 
        let mut outs = vec![];

        // per ogni ricevitore 
        for receiver in &self.receivers {
            // riceve gli impulsi 
            let out = receiver.recv();
            match out {
                Ok(msg) => outs.push(msg),
                Err(e) => return Err(e)
            }
        }

        Ok(outs)
    }

    pub fn run(self) -> JoinHandle<()> {
        // lancia un thread e restituisce un Join Handle, cambiare il return in Result e sostituire il break con un return di RecvError
        thread::spawn(move || {
            loop {
                let res = self.receive();
                match res {
                    Ok(outs) => println!("\t Output Monitor: {:?}", outs),
                    Err(e) => break,
                }
            }
        })
    
    }
}
