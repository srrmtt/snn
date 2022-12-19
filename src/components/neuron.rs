use std::sync::{Arc,};

use super::{spike::Spike};
/*
Classe che contiene l'intelligenza della rete, attraverso i channel i vari neuroni comunicano fra di loro, si utilizzano i Sender
con capacità 0 (canali rendez-vous) in modo da non utilizzare altre memory barrier per sincronizzare input e output con il primo
e ultimo layer.
*/
#[derive(Clone)]
pub struct Neuron {
    // neural network parameters
    v_threshold: f64,
    v_rest: f64,
    v_reset: f64,
    // LIR model function signature, maybe to be generalized
    model: Arc<dyn Fn(i32, i32, f64, f64, f64, Vec<f64>) -> f64 + Send + Sync + 'static>,
    // last 'neuron fired' tension
    v_mem_old: f64,
    // last unit time at which the neuron fired
    ts_1: i32,
    // ts NON è il tempo globale, non è necessario avere un contatore globale perchè la rete ha bisogno solo di differenze temporali (1 - 0) == (12 - 11)
    // ts è un contatore locale al neurone (un'unità indietro rispetto al layer precedente se si considera un tempo t della simulazione)
    ts: i32,
    

    tao: f64,
    // formato: l#n#, dove il primo # indica il numero del layer, mentre il secondo indica il numero del neurone all'interno del layer
    pub position: i32,
}

impl Neuron {
    // Neuron constructor
    pub fn new(
        v_threshold: f64,
        v_rest: f64,
        v_reset: f64,
        tao: f64,
        model: fn(i32, i32, f64, f64, f64, Vec<f64>) -> f64,
        position: i32,
    ) -> Self {
        Self {
            v_threshold,
            v_rest,
            v_reset,
            // at the beginning the net is resting
            v_mem_old: v_rest,
            ts_1: 0,
            ts: 0,
            tao,
            model: Arc::new(model),
            position,
        }
    }
    /*
    
    
    */
    pub fn run(&mut self, weighted_inputs: Vec<f64>) -> Spike {        
        // spike received: true se esiste un valore != 0
        
        let spike_received = !weighted_inputs.is_empty();
        let mut out_spike = 0;
        // 0100001000
        // ts = 0
        // ts_1 = 0
        self.ts += 1;
        if spike_received {
            // se esiste una spike diversa da 0, il neurone comincia l'elaborazione
            
            let out = (*self.model)(
                self.ts,
                self.ts_1,
                self.v_rest,
                self.v_mem_old,
                self.tao,
                // è possibile fare unwrap() perchè altrimenti l'if sarebbe false
                weighted_inputs,
            );
            self.ts_1 = self.ts;
            self.v_mem_old = out;
            // println!("neuron [{}] emits {} at time [{}] --- threshold: {}" , self.name, out, self.ts, self.v_threshold);
            if out > self.v_threshold {
                // se il modello fornisce un valore maggiore della soglia, resetta la tensione di membrana e assegna 1 all'out_spike
                // e aggiorna ts_1 a ts
                out_spike = 1;
                self.v_mem_old = self.v_reset;
            } 
            //println!("{} out at {} : {}", self.to_string(), &self.ts, out_spike);   
        }
        Spike::new(out_spike, Some(self.position))
            
        
    }

    // pub fn to_string(&self) -> String {
        // return format!("[Neuron: {}]", self.position.clone());
    // }
}
