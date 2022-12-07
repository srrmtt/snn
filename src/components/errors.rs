#[derive(Debug)]
pub enum Error {

    /* The input layer is empty */
    EmptyInputLayer,

    /* The input is not yet connected
       Hint: Call the connect_inputs method before running the simulation */
    InconnectedInput,
    /* Output monitor not connected
       Hint: Call the connect output method before running the simulation*/
    InconnectedOutput,
    /* The layer index is out of index */
    OutOfIndexLayer,

    /*The neural network has no layer */
    NoLayerInNN,

    /* Cannot send spike */
    SendError,

    /* Error join */
    JoinError
    
}