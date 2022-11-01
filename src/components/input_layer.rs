use std::thread::{self, JoinHandle};

use std::sync::mpsc::SyncSender;

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
        println!("adding sender to input [{}]", &n_input);
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
        format!("input layer with [{}] inputs.", self.inputs.len())
    }
}
