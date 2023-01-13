mod components;
use components::models::lif;
use components::neural_network::NeuralNetwork;

fn main() {
    println!("-------------------- START -------------------");
    println!("--- Creating neural network from test.json...");
    let nn_res=NeuralNetwork::from_json("./test.json", lif);
    match nn_res{
        Ok(nn) => {
            println!("{}", nn.to_string());
            println!("\t\tDONE.");
            println!("--- Starting simulation...");
            nn.run("output_file.txt");
            println!("\t\tDONE.")
        },
        Err(e) => panic!("{:?}", e)
    }
    

}
