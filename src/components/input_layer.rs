use std::sync::{Arc, Barrier};
use std::thread::{self, JoinHandle};

use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::vec;

use super::spike::Spike;

/*
Conenitore di oggetti Input
*/
pub struct InputLayer {
    // vettore di input
    pub inputs: Vec<Vec<i8>>,
    pub sender: Option<Sender<Vec<Spike>>>,
    pub barrier: Option<Arc<Barrier>>,
    ts: i32,
}

impl InputLayer {
    fn check_inputs(&self) {
        // TODO return a result instead of panicing create an Error for the empty layer and for the non connected layers
        if self.inputs.is_empty() {
            panic!("Input layer is empty, please specify at least a an input vector.");
        }
    }

    pub fn from_file(path: &str, delimiter: char) -> Result<Self, Box<dyn std::error::Error>> {
        // open file return an error if the file is not found
        let mut file = match File::open(Path::new(&path)) {
            Err(err) => return Err(Box::new(err)),
            Ok(f) => f,
        };

        // read file content
        let mut content = String::new();
        // vector of Input structs
        let mut inputs = vec![];
        match file.read_to_string(&mut content) {
            Err(err) => return Err(Box::new(err)),
            Ok(_) => {
                let inputs_str = content.split(delimiter);

                for line in inputs_str {
                    let parse_r: Result<Vec<i8>, ParseIntError> = line
                        .chars()
                        .map(|spike| spike.to_string().parse::<i8>())
                        .collect();
                    match parse_r {
                        Ok(spikes) => {
                            if !spikes.is_empty() {
                                inputs.push(spikes)
                            }
                        }
                        Err(err) => return Err(Box::new(err)),
                    }
                }
            }
        }
        return Ok(Self {
            inputs,
            barrier: None,
            sender: None,
            ts: 0,
        });
    }

    pub fn set_sender(&mut self, tx: Sender<Vec<Spike>>) {
        // add a sender to the n_input-th input object
        // TODO return a result<Ok<()>,Error> for out of bounds error
        // println!("adding sender to input [{}]", &n_input);
        self.sender = Some(tx)
    }
    pub fn set_barrier(&mut self, barrier: Arc<Barrier>) {
        self.barrier = Some(barrier);
    }
    pub fn run(self) -> JoinHandle<()> {
        // vector of thread ids belonging to each spike generator

        // check the inputs status before proceding, modify for handling the result
        self.check_inputs();

        let tid = thread::spawn(move || {
            self.emit_spikes();
        });

        return tid;
    }
    pub fn emit_spikes(mut self) -> () {
        let sender = match self.sender {
            None => panic!(
                "Call the connect before running the simulation\n [Input Layer]: sender is None"
            ),
            Some(s) => s,
        };
        for row_spikes in self.inputs.iter() {
            //println!("[Input Layer]: sending spikes at t_s ({})", self.ts);
            let spikes = row_spikes
                .iter()
                .enumerate()
                .map(|(from, s)| Spike::new(*s, Some(from as i32)))
                .collect();
            match sender.send(spikes) {
                Ok(_) => (),
                Err(e) => panic!("[Input Layer]: {:?}", e)
            }
            self.ts += 1;
            // safe, previous check
            let barrier = self.barrier.as_ref().unwrap();
            //println!("[Input Layer]: Waiting at out barrier.");
            barrier.wait();
        }
        
    }
    pub fn to_string(&self) -> String {
        format!(
            "[Input layer]--> simulation duration: {}\n",
            self.inputs.len()
        )
    }
}
