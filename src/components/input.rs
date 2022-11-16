use std::{fs::File, io::BufReader, io::Read, vec};

use std::sync::mpsc::SyncSender;

/*
The input class contiains the logic to emit single spike to the first neural layer.
*/
pub struct Input {
    spikes: Vec<i8>,
    senders: Vec<SyncSender<i8>>,
    // used just for debugging 
    ts: i8,
}

impl Input {
    pub fn new(spikes: Vec<i8>) -> Self {
        // costruttutore
        Self {
            spikes,
            senders: vec![],
            ts: 0,
        }
    }

    pub fn from_file(filename: &str) -> Result<Self, std::io::Error> {
        // builder method: da un file costruisce un oggetto Input
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();

        buf_reader.read_to_string(&mut content)?;
        // TODO: check di input format and the logic correctness of the input
        let ret = content
            .bytes()
            .into_iter()
            .map(|c| (c - '0' as u8) as i8)
            .collect();

        Ok(Input::new(ret))
    }
    pub fn emit(&self, spike: i8) {
        // emette una spike sul SynchSender
        // TODO return a Result instead of panicking
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
        // logic of the whole input emit spike until the input vector is empty
        while !self.spikes.is_empty() {
            // WRONG! pop() emit spike in the inverse order 
            let spike = self.spikes.pop().unwrap();
            // handle the return result
            self.emit(spike);
            // just for debug
            self.ts += 1;
        }
    }
    pub fn is_empty_sender(&self) -> bool {
        return self.senders.is_empty();
    }

    pub fn add_sender(&mut self, tx: SyncSender<i8>) {
        self.senders.push(tx);
    }

    pub fn to_string(&self) -> String {
        format!("input made of [{}] spikes", self.spikes.len())
    }
}
