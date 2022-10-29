use std::sync::{Arc, Barrier};

use std::sync::mpsc::{RecvError, SyncSender};

use super::synapse::Synapse;

pub struct Neuron {
    // neural network parameters
    v_threshold: f32,
    v_rest: f32,
    v_reset: f32,
    // LIR model function signature, maybe to be generalized
    model: Arc<dyn Fn(i8, i8, f32, f32, f64, Vec<i32>) -> f32 + Send + Sync + 'static>,
    // last 'neuron fired' tension
    v_mem_old: f32,
    // last unit time at which the neuron fired
    ts_1: i8,
    ts: i8,
    // channels' ends
    pub synapses: Vec<Synapse>,
    // neuron output
    pub output: Vec<SyncSender<i8>>,

    tao: f64,

    name: String,
}

impl Neuron {
    // Neuron constructor
    pub fn new(
        v_threshold: f32,
        v_rest: f32,
        v_reset: f32,
        tao: f64,
        model: fn(i8, i8, f32, f32, f64, Vec<i32>) -> f32,
        name: String,
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
            synapses: vec![],
            output: vec![],
            name,
        }
    }
    fn read_spikes(&self) -> Result<Vec<i32>, RecvError> {
        let mut weighted_inputs = vec![];
        println!("[Neuron] ---read at [{}]", self.ts);
        for syanpse in &self.synapses {
            /* 
            if syanpse.get_weight() < 0 && self.ts == 0 {
                continue;
            }*/
            match syanpse.receive() {
                Ok(wi) => {
                    if wi != 0 {
                        weighted_inputs.push(wi)
                    }
                    
                }
                Err(e) => return Err(e),
            }
        }
        return Ok(weighted_inputs);
    }

    pub fn emit_spikes(&self, spike : i8){
        for out in &self.output{
            
            let r = out.send(spike);
            match r {
                Ok(_) => println!("\t\t--> OK"),
                Err(e) => panic!("{}", e)
            }
        }
    }
    pub fn start(&mut self, barrier: Arc<Barrier>) {
        let mut receiving = true;
        
        while receiving {
            let spike_received;
            
            let res_weighted_inputs = self.read_spikes();

            match &res_weighted_inputs {
                Err(_) => {
                    println!("ended input at [{}]", self.ts);
                    receiving=false;
                    spike_received = false;
                }
                Ok(weighted_inputs) => spike_received = weighted_inputs.iter().any(|&wi| wi != 0),
            };
            let mut out_spike = 0;
            if spike_received {
                let out = (*self.model)(
                    self.ts,
                    self.ts_1,
                    self.v_rest,
                    self.v_mem_old,
                    self.tao,
                    res_weighted_inputs.unwrap(),
                );
                if out > self.v_threshold {
                    out_spike = 1;
                    self.ts_1 = self.ts;
                    self.v_mem_old = self.v_reset;
                } 
                //println!("{} out at {} : {}", self.to_string(), &self.ts, out_spike);   
            }
            self.emit_spikes(out_spike);
            self.ts += 1;
            barrier.wait();
            
        }
    }

    pub fn to_string(&self) -> String {
        return format!("[Neuron: {}]", self.name.clone());
    }
}
