mod components;


use components::neural_network::NeuralNetwork;

fn main() {
    println!("-------------------- START -------------------");
    println!("--- Creating neural network from test.json...");
    let mut nn=NeuralNetwork::from_JSON("test.json");
    println!("\t\tDONE.");
    println!("--- Starting simulation...");
    nn.run("output_file.txt"); 
    println!("\t\tDONE.")
}
