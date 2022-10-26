use std::sync::{Arc, Barrier};

use crossbeam::channel::{RecvError, Sender};

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
    pub output: Option<Sender<i8>>,

    tao: f64,
}

impl Neuron {
    // Neuron constructor
    pub fn new(
        v_threshold: f32,
        v_rest: f32,
        v_reset: f32,
        tao: f64,
        model: fn(i8, i8, f32, f32, f64, Vec<i32>) -> f32,
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
            output: Option::None,
        }
    }
    fn read_spikes(&self) -> Result<Vec<i32>, RecvError> {
        let mut weighted_inputs = vec![];
        for syanpse in &self.synapses {
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
    pub fn start(&mut self, barrier: Arc<Barrier>) {
        loop {
            self.ts += 1;
            let res_weighted_inputs = self.read_spikes();

            let spike_received = match &res_weighted_inputs {
                Err(e) => break,
                Ok(weighted_inputs) => weighted_inputs.iter().any(|&wi| wi != 0),
            };
            if spike_received {
                let out = (*self.model)(
                    self.ts,
                    self.ts_1,
                    self.v_rest,
                    self.v_mem_old,
                    self.tao,
                    res_weighted_inputs.unwrap(),
                );
                let out_spike;
                if out > self.v_threshold {
                    out_spike = 1;
                    self.ts_1 = self.ts;
                    self.v_mem_old = self.v_reset;
                } else {
                    out_spike = 0;
                }
                println!("out: {}", out_spike);
                barrier.wait();
            }
        }
    }
}
