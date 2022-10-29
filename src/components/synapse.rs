use std::sync::mpsc::{Receiver, RecvError};

pub struct Synapse {
    weight: i32,
    from: Receiver<i8>,
}

impl Synapse {
    pub fn new(weight: i32, from: Receiver<i8>) -> Self {
        Self { weight, from }
    }

    pub fn receive(&self) -> Result<i32, RecvError> {
        let msg = self.from.recv();
        match msg {
            Ok(spike) => Ok(spike as i32 * self.weight),
            Err(err) => Err(err),
        }
    }

    pub fn get_weight(&self) -> i32{
        self.weight
    }
}
