mod components;

use libm::exp;
use std::vec;

use components::neural_network::NeuralNetwork;

use crate::components::output::OutputMonitor;
use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Value {
    thresholds: Vec<Vec<f32>>,
    rest_potential: f32,
    reset_potential: f32,
    tau: f64,
    intra_layer_weights: Vec<Vec<Vec<f32>>>
    }

fn lif(ts: i8, ts_1: i8, v_rest: f32, v_mem_old: f32, tao: f64, weights: Vec<i32>) -> f32 {
    let k = -(ts - ts_1) as f64 / tao;

    let exponential = exp(k) as f32;

    let v_mem = v_rest + (v_mem_old - v_rest) * exponential;

    let weight = weights.iter().fold(0, |sum, x| sum + x) as f32;
    return v_mem + weight;
}

fn main() {
    let file = File::open("weights.txt");
    let the_file = File::open("test.json").unwrap();
    let input: Value = serde_json::from_reader(the_file).expect("JSON was not well-formatted");
    let output_file = "out.txt";

    let mut nn = NeuralNetwork::new(input.v_threshold, input.v_rest, input.v_reset, input.tao, lif, &[2,2]);
    let input_w = vec![vec![10, 1], vec![5, 1]];

    
    let files = [
        "./data/input1.txt",
        "./data/input2.txt"

    ];
    let single_input_w = vec![vec![10, 1],vec![10, 1]];
  
    let om = OutputMonitor::new(output_file);
    nn.connect_inputs(&files, single_input_w);
    println!("{}", &nn.to_string());
    println!("inputs: connected");
   // let l1_internal_w = vec![vec![None, Some(-3)], vec![Some(-1), None]];
    nn.connect(0, 0, vec![vec![None, Some(-1)], vec![Some(-2), None]]);
    nn.connect(0, 1, vec![vec![None, Some(-1)], vec![Some(-2), None]]);

    //println!("internal connection: done");
    nn.connect_output(om);
    nn.run();
}
