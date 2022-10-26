use std::thread::{self, JoinHandle};

use crossbeam::channel::Sender;

use super::input::Input;

pub struct InputLayer {
    // [ [ 00001101001 ], [010001001]]
    pub inputs: Vec<Input>,
}

impl InputLayer {
    pub fn new() -> Self {
        Self { inputs: vec![] }
    }
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

    pub fn add(&mut self, input: Input) {
        self.inputs.push(input);
    }

    pub fn set_input_sender(&mut self, n_input: usize, tx: Sender<i8>) {
        self.inputs[n_input].set_sender(tx);
    }
    pub fn emit_spikes(mut self) -> Vec<JoinHandle<()>> {
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
}
