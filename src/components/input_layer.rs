use std::thread::{self, JoinHandle};

use std::num::ParseIntError;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::SyncSender;
use std::vec;

use super::input::Input;

/*
Conenitore di oggetti Input
*/
pub struct InputLayer {
    // vettore di input
    pub inputs: Vec<Input>,
}

impl InputLayer {
    fn check_inputs(&self) {
        // TODO return a result instead of panicing create an Error for the empty layer and for the non connected layers
        if self.inputs.is_empty() {
            panic!("Input layer is empty, please specify at least a file.");
        }
        for input in &self.inputs {
            if input.is_empty_sender() {
                panic!("Call the connect_inputs method of the neural network class before running the simulation.")
            }
        }
    }

    pub fn from_file(path: &str, delimiter: char) -> Result<Self, Box<dyn std::error::Error>>{
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
                
                for line in inputs_str{
                    let parse_r : Result<Vec<i8>, ParseIntError>= line.chars().map(|spike| spike.to_string().parse::<i8>()).collect();
                    match parse_r {
                        Ok(spikes) => {
                            if !spikes.is_empty() {
                                inputs.push(Input::new(spikes))
                            }
                        },
                        Err(err) => return  Err(Box::new(err)),
                    }
                }
                
                
            }
        }
        return Ok(Self { inputs })
    }

    //TODO change the return in a Result for handling the reading file error
    pub fn from_files(filenames: &[&str]) -> Self {
        // create an Input layer from a file vector reference

        let mut inputs = vec![];

        for filename in filenames {
            let r = Input::from_file(*filename);
            match r {
                Ok(input) => inputs.push(input),
                Err(e) => panic!("Error during reading: {}: {:?}", *filename, e),
            }
        }
        // TODO check the length of each input, if the lengths differ return an error
        Self { inputs }
    }

    pub fn add_sender_to(&mut self, n_input: usize, tx: SyncSender<i8>) {
        // add a sender to the n_input-th input object 
        // TODO return a result<Ok<()>,Error> for out of bounds error  
        // println!("adding sender to input [{}]", &n_input);
        self.inputs[n_input].add_sender(tx);
    }

    pub fn emit_spikes(self) -> Vec<JoinHandle<()>> {
        // vector of thread ids belonging to each spike generator
        let mut tids = vec![];
        // check the inputs status before proceding, modify for handling the result 
        self.check_inputs();

        for input in self.inputs {
            // spawn a thread for each input file
            let child = thread::spawn(move || {
                input.run();
            });
            tids.push(child);
        }
        return tids;
    }

    pub fn to_string(&self) -> String {
        let res = format!("input layer with [{}] inputs.\n", self.inputs.len());
        // for input in &self.inputs {
        //     res = format!("{}\n\t{}", res, input.to_string());
        // }
        res
    }
}
