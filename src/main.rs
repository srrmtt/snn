use std::fs::File;
use std::io::{BufReader, Read};
use std::vec;
use libm::exp;
use crossbeam::channel::{self, Receiver, Sender};

pub struct NeuralLayer{
    neurons: Vec<Neuron>
}

impl NeuralLayer {
    pub fn new() -> Self{
        NeuralLayer { neurons: vec![] }
    }

    pub fn add_neurons(&mut self, new_neurons: &mut Vec<Neuron>){
        self.neurons.append(new_neurons);
    }

    pub fn add_neuron(&mut self, neuron: Neuron){
        self.neurons.push(neuron);
    }
}
pub struct NeuralNetwork{
    // neural network parameters
    v_threshold : f32,
    v_rest : f32, 
    v_reset : f32,
    // LIR model function signature, maybe to be generalized
    model : Box<dyn Fn( i8, i8, f32, f32, f32, Vec<i8>) -> f32>,

    input_reader : InputReader,

    layers : Vec<NeuralLayer>,


}

impl NeuralNetwork {
    pub fn new(v_threshold : f32, v_rest : f32, v_reset : f32, model : fn( i8, i8, f32, f32, f32, Vec<i8>) -> f32, npl : &[i8]) -> Self{
        let mut layers = vec![];
        
        for n_neurons in npl {
            let mut nl = NeuralLayer::new();
            for _ in 0..*n_neurons{
                nl.add_neuron(Neuron::new(v_threshold, v_rest, v_reset, model))
            }
            layers.push(nl);
        }

        Self { 
            v_threshold,
            v_rest,
            v_reset, 
            model: Box::new(model), 
            input_reader: InputReader::empty_reader(), 
            layers
        }
    }

    pub fn run(&mut self, input_files: &[&str], output_file: &str) -> Result<File, std::io::Error>{
        self.input_reader = InputReader::from_files(input_files);
        File::create(output_file)
    }

    pub fn connect(&mut self, l1: i8, l2: i8, weights: Vec<Vec<i32>>){
         return;
    }

    pub fn connect_inputs(&mut self, filenames: &[&str], weights: Vec<i32>){
        if self.layers.len() == 0 {
            panic!("Cannot link input with first layer, if the layer does not exist.")
        }
        for filename in filenames{
            let v = InputReader::read_file(filename);
            match v {
                Ok(content) => self.input_reader.inputs.push(content),
                Err(e) => panic!("Error: {:?}", e) 
            }
        }

        

    }

}

pub struct Neuron {
    // neural network parameters
    v_threshold : f32,
    v_rest : f32, 
    v_reset : f32,
    // LIR model function signature, maybe to be generalized
    model : Box<dyn Fn( i8, i8, f32, f32, f32, Vec<i8>) -> f32>,
    // last 'neuron fired' tension 
    v_mem_old : f32,
    // last unit time at which the neuron fired
    ts_1 : i8,
    // channels' ends
    synapses: Vec<Receiver<i8>>,
    // inhibitory channels
    inib_channels: Vec<Receiver<i8>>,
    // neuron output
    output: Option<Sender<i32>>,
    // income synapses weight from the exitatory layer
    exitatory_weights: Vec<i32>,
    inhibitory_weights: Vec<i32>
}

impl Neuron {
    fn new(v_threshold : f32, v_rest : f32, v_reset : f32, model: fn( i8, i8, f32, f32, f32, Vec<i8>) -> f32) -> Self{
        Self { 
            v_threshold, 
            v_rest, 
            v_reset, 
            v_mem_old: v_rest,
            ts_1: 0, 
            model: Box::new(model),
            synapses: vec![],
            inib_channels: vec![],
            output: Option::None,
            exitatory_weights: vec![],
            inhibitory_weights: vec![], 
        }
    }

    
}


pub struct InputReader {
    // [ [ 00001101001 ], [010001001]]
    inputs : Vec<Vec<i8>>,
    out: Option<Sender<Vec<i8>>>,
}

impl InputReader {
    pub fn empty_reader() -> Self {
        Self { 
            inputs: vec![],
            out: Option::None,
        }
    }
    pub fn read_file(filename: &str) -> Result<Vec<i8>, std::io::Error>{
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();

        buf_reader.read_to_string(&mut content)?;

        let ret = content.bytes().into_iter().map(|c| (c - '0' as u8) as i8).collect();

        Ok(ret)
        
    }
    fn from_files(filenames : &[&str]) -> Self{
        let mut inputs: Vec<Vec<i8>> = vec![];
        for f in filenames{
            match InputReader::read_file(f) {
                Err(e) => {
                    panic!("Unable to read file: {}.", e);
                },
                Ok(v) => {
                    inputs.push(v);
                }
            }
        }

        Self { inputs, out: Option::None }
    }
    pub fn print(&self){
        for i in &self.inputs{
            println!("{:?}", i);
        }
    }
}

fn main() {
    
    let model = |ts: i8, ts_1: i8, v_rest: f32, v_mem_old: f32, tao: f64, weights: Vec<i8>| -> f32{
        let k = - (ts - ts_1) as f64 / tao;
        
        let exponential = exp(k) as f32;

        let v_mem = v_rest + (v_mem_old - v_rest) * exponential;

        let weight = weights.iter().fold(0, |sum, x| sum + x) as f32;
        return v_mem + weight; 
          
    };
    let files = [".\\data\\input1.txt", ".\\data\\input2.txt", ".\\data\\input3.txt"];
    let ir = InputReader::from_files(&files );
    println!("--- testing input --- ");
    ir.print()
    
}
