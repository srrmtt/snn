#[derive(Clone,Copy)]
pub struct Spike{
     pub output: i8,
    pub n_neuron: Option<i32>
}

impl Spike{
    pub fn new(output: i8, n_neuron: Option<i32>) -> Self {
        Self {
            output, n_neuron
        }

    
    }
}