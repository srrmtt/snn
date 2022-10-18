use std::fs::File;
use std::io::{BufReader, Read};
use libm::exp;
pub struct Neuron {
    v_threshold : f32,
    v_rest : f32, 
    v_reset : f32,

    model : Box<dyn Fn( i8, i8, f32, f32, f32, Vec<i8>) -> f32>
}

impl Neuron {
    fn new(v_threshold : f32, v_rest : f32, v_reset : f32, model: fn( i8, i8, f32, f32, f32, Vec<i8>) -> f32) -> Self{
        Self { 
            v_threshold, 
            v_rest, 
            v_reset, 
            model: Box::new(model) 
        }
    }
}


pub struct InputReader {
    inputs : Vec<Vec<i8>>
}

impl InputReader {
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

        Self { inputs }
    }
    pub fn read(&self){
        for i in &self.inputs{
            println!("{:?}", i);
        }
    }
}

fn main() {
    println!("--- testing input --- ");
    let model = |ts: i8, ts_1: i8, v_rest: f32, v_mem_old: f32, tao: f64, weights: Vec<i8>| -> f32{
        let k = - (ts - ts_1) as f64 / tao;
        
        let exponential = exp(k) as f32;

        let v_mem = v_rest + (v_mem_old - v_rest) * exponential;

        let weight = weights.iter().fold(0, |sum, x| sum + x) as f32;
        return v_mem + weight; 
          
    };
    let files = [".\\data\\input1.txt", ".\\data\\input2.txt", ".\\data\\input3.txt"];
    let ir = InputReader::from_files(&files );
    ir.read()
    
}
