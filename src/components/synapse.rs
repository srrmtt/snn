use std::sync::mpsc::{Receiver};

use super::{spike::Spike, errors::SNNError};
/*
 Unità logica contenuta nei neuroni per ricevere le spike in ingresso, costituita da un receiver e da un peso associato alla connessione.
*/
pub struct Synapse {
    weight: f64,
    rec: Receiver<Spike>,
    
}

impl Synapse {
    pub fn new(weight: f64, rec: Receiver<Spike>) -> Self {
        Self { weight, rec}
    }

    pub fn receive(&self) -> Result<f64, SNNError> {
        // receive a single spike at a time
        let msg = self.rec.recv();
        match msg {
            Ok(spike) => Ok(spike.output as f64 * self.weight),
            Err(_) => Err(SNNError::EmptyChannelError("Call the connect before calling the receive method.".to_string())),
        }
    }

    pub fn get_weight(&self) -> f64{
        self.weight
    }
}
