mod components;

use std::{time::{Instant, Duration}, array, thread, sync::{Barrier, Mutex}};

use components::neural_network::NeuralNetwork;

fn main() {
    println!("-------------------- START -------------------");
    println!("--- Creating neural network from test.json...");
    let nn=NeuralNetwork::from_json("./test.json");
    //let nn=NeuralNetwork::from_JSON("./data/simple_test.json");
    println!("{}", nn.to_string());
    println!("\t---------- DONE -----------");
    println!("--- Starting simulation...");
    let now = Instant::now();
    nn.run("output_file.txt");
    println!("time: {}", now.elapsed().as_secs_f64());
    println!("\t\tDONE.")

    
}
