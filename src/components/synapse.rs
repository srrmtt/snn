use std::sync::mpsc::{Receiver, RecvError};
/*
 Unità logica contenuta nei neuroni per ricevere le spike in ingresso, costituita da un receiver e da un peso associato alla connessione.
*/
pub struct Synapse {
    weight: f64,
    from: Receiver<i8>,
}

impl Synapse {
    pub fn new(weight: f64, from: Receiver<i8>) -> Self {
        Self { weight, from }
    }

    pub fn receive(&self) -> Result<f64, RecvError> {
        // receive a single spike at a time
        let msg = self.from.recv();
        match msg {
            Ok(spike) => Ok(spike as f64 * self.weight),
            Err(err) => Err(err),
        }
    }

    pub fn get_weight(&self) -> f64{
        self.weight
    }
}
