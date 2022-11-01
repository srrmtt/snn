use std::sync::mpsc::{Receiver, RecvError};
/*
 Unità logica contenuta nei neuroni per ricevere le spike in ingresso, costituita da un receiver e da un peso associato alla connessione.
*/
pub struct Synapse {
    weight: i32,
    from: Receiver<i8>,
}

impl Synapse {
    pub fn new(weight: i32, from: Receiver<i8>) -> Self {
        Self { weight, from }
    }

    pub fn receive(&self) -> Result<i32, RecvError> {
        // receive a single spike at a time
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
