use std::thread::{self, JoinHandle};

use std::sync::mpsc::SyncSender;

use super::input::Input;

pub struct InputLayer {
    // [ [ 00001101001 ], [010001001]]
    pub inputs: Vec<Input>,
}

impl InputLayer {
    fn check_inputs(&self) {
        if self.inputs.is_empty() {
            panic!("Input layer is empty, please specify at least a file.");
        }
        for input in &self.inputs {
            if input.is_empty_sender() {
                panic!("Call the connect_inputs method of the neural network class before running the simulation.")
            }
        }
    }

    pub fn from_files(filenames: &[&str]) -> Self {
        let mut inputs = vec![];

        for filename in filenames {
            let r = Input::from_file(*filename);
            match r {
                Ok(input) => inputs.push(input),
                Err(e) => panic!("Error during reading: {}: {:?}", *filename, e),
            }
        }

        Self { inputs }
    }

    pub fn add_sender_to(&mut self, n_input: usize, tx: SyncSender<i8>) {
        println!("adding sender to input [{}]", &n_input);
        self.inputs[n_input].add_sender(tx);
    }

    pub fn emit_spikes(self) -> Vec<JoinHandle<()>> {
        // vector of thread ids belonging to each spike generator
        let mut tids = vec![];
        // check the inputs status before proceding
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
