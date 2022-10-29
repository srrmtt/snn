use std::{sync::mpsc::{Receiver, RecvError}, thread::{JoinHandle, self}};

pub struct OutputMonitor {
    receivers: Vec<Receiver<i8>>,
    filename: String,
    outputs: Vec<Vec<i8>>,
}

impl OutputMonitor {
    pub fn new(filename: &str) -> Self {
        Self {
            receivers: vec![],
            outputs: vec![],
            filename: filename.to_string(),
        }
    }

    pub fn add_receiver(&mut self, receiver: Receiver<i8>) {
        self.receivers.push(receiver);
    }

    pub fn receive(&self) -> Result<Vec<i8>, RecvError>{
        let mut outs = vec![];
        for receiver in &self.receivers {
            let out = receiver.recv();
            match out {
                Ok(msg) => outs.push(msg),
                Err(e) => return Err(e)
            }
        }

        Ok(outs)
    }

    pub fn run(self) -> JoinHandle<()> {
        
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
