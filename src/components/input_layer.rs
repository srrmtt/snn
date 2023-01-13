use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::path::Path;
use std::sync::{Barrier, Arc};
use std::sync::mpsc::Sender;
use std::thread::{JoinHandle, self};

use super::errors::SNNError;
use super::input::{Input};
use super::spike::Spike;

/*
Conenitore di oggetti Input
*/
pub struct InputLayer {
    // vettore di input
    pub inputs: Vec<Input>,
}

impl InputLayer {
    fn check_inputs(&self) -> Result<(), SNNError>{
        if self.inputs.is_empty() {
            // controlla che ci siano delle spike da emettere
            return Err(SNNError::EmptyInputLayer("Input layer is empty, please specify at least a file.".to_string()));
        }
        for input in &self.inputs {
            // controlla che ogni input sia connesso a almeno a un neurone
            if input.is_empty_sender() {
                return Err(SNNError::EmptyChannelError("Call the connect_inputs method of the neural network class before running the simulation.".to_string()))
            }
        }
        Ok(())
    }

    pub fn from_file(path: &str, delimiter: char) -> Result<Self, SNNError>{
        // riceve un file e un delimitatore per ogni delimitatore crea un array di spike

        // open file return an error if the file is not found
        let mut file = match File::open(Path::new(&path)) {
            Err(_) => return Err(SNNError::FileError(format!("Cannot open file {path}."))),
            Ok(f) => f,
        };

        // read file content
        let mut content = String::new();
        // vector of Input structs
        let mut inputs = vec![];

        match file.read_to_string(&mut content) {
            Err(err) => return Err(SNNError::FileError(format!("File :{path}\nERROR:{err}"))),
            Ok(_) => {
                let inputs_str = content.split(delimiter);
                
                for line in inputs_str{
                    // ogni linea del file corrisponde a una struttura Input dell'input layer
                    let parse_r : Result<Vec<i8>, ParseIntError>= line.chars().map(|spike| spike.to_string().parse::<i8>()).collect();
                    match parse_r {
                        Ok(spikes) => {
                            if !spikes.is_empty() {
                                inputs.push(Input::new(spikes))
                            }
                        },
                        Err(_) => return  Err(SNNError::BadFormatError("Parse Error, check your input file".to_string())),
                    }
                }
                
                
            }
        }
        return Ok(Self { inputs })
    }

    pub fn add_sender_to(&mut self, n_input: usize, tx: Sender<Spike>) {
        // add a sender to the n_input-th input object 
        self.inputs[n_input].add_sender(tx);
    }

    pub fn emit_spikes(self) -> Vec<JoinHandle<()>> {
        // vector of thread ids belonging to each spike generator
        let mut tids = vec![];
        // check the inputs status before proceding, modify for handling the result 
        let res = self.check_inputs();
        if res.is_err(){
            panic!("{:?}", res)
        }
        // a ogni input corrisponde un thread
        let n_thread = self.inputs.len();
        let barrier = Arc::new(Barrier::new(n_thread));


        for input in self.inputs {
            let c = Arc::clone(&barrier);
            // spawn a thread for each input file
            let child = thread::spawn(move || {
                input.run(c);
            });
            tids.push(child);
        }
        return tids;
    }

    pub fn to_string(&self) -> String {
        let res = format!("input layer with [{}] inputs.\n", self.inputs.len());
        res
    }
}
