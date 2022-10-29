use std::{fs::File, io::BufReader, io::Read, vec};

use std::sync::mpsc::SyncSender;

pub struct Input {
    spikes: Vec<i8>,
    senders: Vec<SyncSender<i8>>,
    ts: i8,
}

impl Input {
    pub fn new(spikes: Vec<i8>) -> Self {
        Self {
            spikes,
            senders: vec![],
            ts: 0,
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
    pub fn emit(&self, spike: i8) {
        println!("[Input] ---sending: {} at ts: [{}]", spike, self.ts);

        for input in &self.senders {
            // TODO handle SendError
            let r = input.send(spike);
            match r {
                Ok(()) => continue,
                Err(e) => panic!("Error {}", e),
            }
        }
    }
    pub fn run(mut self) {
        while !self.spikes.is_empty() {
            let spike = self.spikes.pop().unwrap();
            self.emit(spike);
            self.ts += 1;
        }
    }
    pub fn is_empty_sender(&self) -> bool {
        return self.senders.is_empty();
    }

    pub fn add_sender(&mut self, tx: SyncSender<i8>) {
        self.senders.push(tx);
    }
}
