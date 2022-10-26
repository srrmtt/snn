use std::{fs::File, io::BufReader, io::Read};

use crossbeam::channel::Sender;

pub struct Input {
    spikes: Vec<i8>,
    sender: Option<Sender<i8>>,
}

impl Input {
    pub fn new(spikes: Vec<i8>) -> Self {
        Self {
            spikes,
            sender: None,
        }
    }

    pub fn from_file(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();

        buf_reader.read_to_string(&mut content)?;

        let ret = content
            .bytes()
            .into_iter()
            .map(|c| (c - '0' as u8) as i8)
            .collect();

        Ok(Input::new(ret))
    }

    pub fn run(mut self) {
        while !self.spikes.is_empty() {
            match &self.sender {
                Some(s) => {
                    let message = self.spikes.pop().unwrap();
                    let r = s.send(message);
                    match r {
                        Err(e) => break,
                        Ok(()) => continue,
                    }
                }
                None => {
                    unreachable!("input --> run() method, should never happen because it's tested.")
                }
            }
        }
    }
    pub fn is_empty_sender(&self) -> bool {
        return self.sender.is_none();
    }

    pub fn set_sender(&mut self, tx: Sender<i8>) {
        self.sender = Some(tx);
    }
}
