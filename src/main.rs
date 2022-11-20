mod components;


use components::neural_network::NeuralNetwork;

use crate::components::output::OutputMonitor;

fn main() {
    let output_file = "out.txt"; 

    let mut nn=NeuralNetwork::from_JSON("test.json");
  
   /* let input_w = vec![vec![10, 1], vec![5, 1]];

    
    let single_input_w = vec![vec![10, 1],vec![10, 1]];
  
    nn.connect_inputs(&files, single_input_w);
    println!("{}", &nn.to_string());
    println!("inputs: connected");
   // let l1_internal_w = vec![vec![None, Some(-3)], vec![Some(-1), None]];
    nn.connect(0, 0, vec![vec![None, Some(-1)], vec![Some(-2), None]]);
    nn.connect(0, 1, vec![vec![None, Some(-1)], vec![Some(-2), None]]);
  */
    //println!("internal connection: done");
    let om = OutputMonitor::new(output_file);

    nn.connect_output(om);
    nn.run(); 
}
