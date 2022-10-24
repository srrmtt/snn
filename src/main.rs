mod components;

use components::input_layer::InputLayer;
use libm::exp;
use std::vec;

use components::neural_network::NeuralNetwork;

fn lif(ts: i8, ts_1: i8, v_rest: f32, v_mem_old: f32, tao: f64, weights: Vec<i32>) -> f32 {
    let k = -(ts - ts_1) as f64 / tao;

    let exponential = exp(k) as f32;

    let v_mem = v_rest + (v_mem_old - v_rest) * exponential;

    let weight = weights.iter().fold(0, |sum, x| sum + x) as f32;
    return v_mem + weight;
}
fn main() {
    let v_threshold = 1.0;
    let v_rest = 0.6;
    let v_reset = 0.4;

    let tao = 0.04;

    let output_file = "out.txt";

    let files = [
        ".\\data\\input1.txt",
        ".\\data\\input2.txt",
        ".\\data\\input3.txt",
    ];
    let mut ir = InputLayer::from_files(&files);
    println!("--- testing input --- ");
    ir.print();

    let mut nn = NeuralNetwork::new(v_threshold, v_rest, v_reset, tao, lif, &[1]);
    let input_w = vec![vec![10, 20, 30]];

    nn.connect_inputs(&files, input_w);
    println!("inputs: connected");
    nn.run(output_file);
    // TODO: scrivere il neruone come thread e farlo comunicare con gli input reader, temporizzare l'emissione di un
    // input con l'output
}
