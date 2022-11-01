mod components;

use libm::exp;
use std::vec;

use components::neural_network::NeuralNetwork;

use crate::components::output::OutputMonitor;

fn lif(ts: i8, ts_1: i8, v_rest: f32, v_mem_old: f32, tao: f64, weights: Vec<i32>) -> f32 {
    let k = -(ts - ts_1) as f64 / tao;

    let exponential = exp(k) as f32;

    let v_mem = v_rest + (v_mem_old - v_rest) * exponential;

    let weight = weights.iter().fold(0, |sum, x| sum + x) as f32;
    return v_mem + weight;
}
fn main() {
    let v_threshold = 5.0;
    let v_rest = 0.6;
    let v_reset = 0.4;

    let tao = 0.04;

    let output_file = "out.txt";
    
    let files = [
        "./data/input1.txt",
        "./data/input2.txt",
        //"./data/input3.txt",
    ];

    let mut nn = NeuralNetwork::new(v_threshold, v_rest, v_reset, tao, lif, &[2]);
    let input_w = vec![vec![10, 1], vec![5, 1]];

    
    let files = [
        "./data/input1.txt"
    ];
    let single_input_w = vec![vec![10, 1]];
  
    let om = OutputMonitor::new(output_file);
    nn.connect_inputs(&files, single_input_w);
    println!("{}", &nn.to_string());
    println!("inputs: connected");
    let l1_internal_w = vec![vec![None, Some(-3)], vec![Some(-1), None]];
    // nn.connect(0, 1, vec![vec![Some(1)], vec![Some(2)]]);
    nn.connect(0, 0, vec![vec![None, Some(-1)], vec![Some(-2), None]]);
    //println!("internal connection: done");
    nn.connect_output(om);
    nn.run();
    // TODO: scrivere il neruone come thread e farlo comunicare con gli input reader, temporizzare l'emissione di un
    // input con l'output
}
