#[derive(Debug)]


pub enum SNNError {

    /* The input layer is empty */
    EmptyInputLayer(String),

    /* The input is not yet connected
       Hint: Call the connect_inputs method before running the simulation */
    InconnectedInput(String),
    /* Output monitor not connected
       Hint: Call the connect output method before running the simulation*/
    InconnectedOutput(String),
    /* The layer index is out of index */
    OutOfIndexError(String),

    /* Empty channel error*/
    EmptyChannelError(String),

    /* Bad formatting file error */
    BadFormatError(String),

    FileError(String)
    
}
