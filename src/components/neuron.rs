use crossbeam::channel::{Receiver, Sender};

pub struct Neuron {
    // neural network parameters
    v_threshold: f32,
    v_rest: f32,
    v_reset: f32,
    // LIR model function signature, maybe to be generalized
    model: Box<dyn Fn(i8, i8, f32, f32, f64, Vec<i32>) -> f32>,
    // last 'neuron fired' tension
    v_mem_old: f32,
    // last unit time at which the neuron fired
    ts_1: i8,
    ts: i8,
    // channels' ends
    pub synapses: Option<Receiver<Vec<i8>>>,
    // inhibitory channels
    inib_channels: Vec<Receiver<i8>>,
    // neuron output
    output: Option<Sender<i32>>,
    // income synapses weight from the exitatory layer
    pub exitatory_weights: Vec<i32>,
    inhibitory_weights: Vec<i32>,

    tao: f64,
}

impl Neuron {
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
            v_mem_old: v_rest,
            ts_1: 0,
            ts: 0,
            tao,
            model: Box::new(model),
            synapses: None,
            inib_channels: vec![],
            output: Option::None,
            exitatory_weights: vec![],
            inhibitory_weights: vec![],
        }
    }

    pub fn start(&self) {
        let mut spike = false;
        loop {
            match &self.synapses {
                Some(rx) => {
                    let message = rx.recv();
                    match message {
                        Ok(inputs) => {
                            let mut weighted_inputs = vec![];
                            for (i, input) in inputs.iter().enumerate() {
                                if *input == 1 {
                                    spike = true;
                                }
                                weighted_inputs.push(*input as i32 * self.exitatory_weights[i]);
                            }
                            if spike {
                                let out = (self.model)(
                                    self.ts,
                                    self.ts_1,
                                    self.v_rest,
                                    self.v_mem_old,
                                    self.tao,
                                    weighted_inputs,
                                );
                                print!("{:?}", out);
                            }
                        }
                        Err(e) => {
                            panic!("{:?}", e);
                        }
                    }
                }
                None => {}
            }
        }
    }
}
