use std::{
    fs::File,
    io::{BufReader, Read},
    thread::{self, JoinHandle},
};

use crossbeam::channel::Sender;

pub struct InputLayer {
    // [ [ 00001101001 ], [010001001]]
    pub inputs: Vec<Vec<i8>>,
    pub out: Option<Sender<Vec<i8>>>,
}

impl InputLayer {
    pub fn empty_reader() -> Self {
        Self {
            inputs: vec![],
            out: Option::None,
        }
    }
    pub fn read_file(filename: &str) -> Result<Vec<i8>, std::io::Error> {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();

        buf_reader.read_to_string(&mut content)?;

        let ret = content
            .bytes()
            .into_iter()
            .map(|c| (c - '0' as u8) as i8)
            .collect();

        Ok(ret)
    }
    pub fn from_files(filenames: &[&str]) -> Self {
        let mut inputs: Vec<Vec<i8>> = vec![];
        for f in filenames {
            match InputLayer::read_file(f) {
                Err(e) => {
                    panic!("Unable to read file: {}.", e);
                }
                Ok(v) => {
                    inputs.push(v);
                }
            }
        }

        Self {
            inputs,
            out: Option::None,
        }
    }
    pub fn print(&self) {
        for i in &self.inputs {
            println!("{:?}", i);
        }
    }

    pub fn emit_spikes(mut self) -> JoinHandle<i8> {
        let child = thread::spawn(move || {
            let sender;
            match &self.out {
                Some(s) => {
                    sender = s;
                }
                None => {
                    println!("Input layer is not connect with the network, call the connect_inputs of NeuralNetwork class to correct this error.");
                    return -1;
                }
            }
            let mut end = false;
            while !end {
                let mut spikes = vec![];
                for i in 0..self.inputs.len() {
                    match self.inputs[i].pop() {
                        Some(spike) => spikes.push(spike),
                        None => end = true,
                    }
                }
                println!("sending... {:?}", &spikes);
                let result = sender.send(spikes);
                match result {
                    Err(e) => println!("{:?}", e),
                    _ => (),
                }
            }
            return 1;
        });

        return child;
    }
}
