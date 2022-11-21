use std::sync::{Arc, Barrier};

use std::sync::mpsc::{RecvError, SyncSender};

use super::synapse::Synapse;
/*
Classe che contiene l'intelligenza della rete, attraverso i channel i vari neuroni comunicano fra di loro, si utilizzano i SyncSender
con capacità 0 (canali rendez-vous) in modo da non utilizzare altre memory barrier per sincronizzare input e output con il primo
e ultimo layer.
*/
pub struct Neuron {
    // neural network parameters
    v_threshold: f64,
    v_rest: f64,
    v_reset: f64,
    // LIR model function signature, maybe to be generalized
    model: Arc<dyn Fn(i32, i32, f64, f64, f64, Vec<f64>) -> f64 + Send + Sync + 'static>,
    // last 'neuron fired' tension
    v_mem_old: f64,
    // last unit time at which the neuron fired
    ts_1: i32,
    // ts NON è il tempo globale, non è necessario avere un contatore globale perchè la rete ha bisogno solo di differenze temporali (1 - 0) == (12 - 11)
    // ts è un contatore locale al neurone (un'unità indietro rispetto al layer precedente se si considera un tempo t della simulazione)
    ts: i32,
    // channels' ends with associated weight 
    pub synapses: Vec<Synapse>,
    // neuron output
    pub output: Vec<SyncSender<i8>>,

    tao: f64,
    // formato: l#n#, dove il primo # indica il numero del layer, mentre il secondo indica il numero del neurone all'interno del layer
    name: String,
}

impl Neuron {
    // Neuron constructor
    pub fn new(
        v_threshold: f64,
        v_rest: f64,
        v_reset: f64,
        tao: f64,
        model: fn(i32, i32, f64, f64, f64, Vec<f64>) -> f64,
        name: String,
    ) -> Self {
        Self {
            v_threshold,
            v_rest,
            v_reset,
            // at the beginning the net is resting
            v_mem_old: v_rest,
            ts_1: 0,
            ts: 0,
            tao,
            model: Arc::new(model),
            synapses: vec![],
            output: vec![],
            name,
        }
    }
    fn read_spikes(&self) -> Result<Vec<f64>, RecvError> {
        // legge gli impulsi provenienti dal layer precedente (sia neurale che di input)

        // vettore che contiene (w_i * s_i) dove s_i è 0 o 1 e w_i è il peso della connessione
        let mut weighted_inputs = vec![];
        
        // per ogni connessione in ingresso 
        for syanpse in &self.synapses {
             
            // a ts = 0 (inizio della simulazione) nessun neurone scatta e quindi non deve aspettare sulle sinapsi inibitorie
            if syanpse.get_weight() < 0.0 && self.ts == 0 {
                continue;
            }

            // riceve gli input nella forma di Result<RecvError, Ok(s_i * w_i)
            match syanpse.receive() {
                // wi: weighted input
                Ok(wi) => {
                    // considera solo gli input != 0
                    if wi != 0.0 {
                        weighted_inputs.push(wi)
                    }
                    
                }
                Err(e) => return Err(e),
            }
        }
        // println!("[Neuron {}] --- {:?} read at [{}]",&self.name, weighted_inputs, self.ts);
        return Ok(weighted_inputs);
    }

    pub fn emit_spikes(&self, spike : i8){
        // invia 0 o 1 ai neuroni successivi

        // per ogni connessione in uscita 
        for out in &self.output{
            // invia la spike
            let r = out.send(spike);
            // TODO vedere se c'è un modo di gestire solo il ramo Err, l'Ok non dovrebbe fare nulla  
            match r {
                Ok(_) => {},
                Err(e) => panic!("{}", e)
            }
        }
    }
    pub fn run(&mut self, barrier: Arc<Barrier>) {
        // riceve uno smart pointer a barrier per sincronizzarsi con gli altri neuroni

        // receiving: true se il layer precedente invia Result Ok, false altrimenti (fine della trasimissione, canale chiuso)
        let mut receiving = true;
        
        while receiving {
            // spike received: true se esiste un valore != 0
            let spike_received;
            // vettore di ingressi pesati provenienti dai neuroni di ingresso 
            let res_weighted_inputs = self.read_spikes();

            match &res_weighted_inputs {
                Err(_) => {
                    // fine della connessione, estremità in ingresso chiusa 
                    println!("ended input at [{}]", self.ts);
                    receiving=false;
                    spike_received = false;
                }
                // true se esiste un input != 0 
                Ok(weighted_inputs) => spike_received = !weighted_inputs.is_empty(),
            };
            let mut out_spike = 0;
            // 0100001000
            // ts = 0
            // ts_1 = 0
            self.ts += 1;
            if spike_received {
                // se esiste una spike diversa da 0, il neurone comincia l'elaborazione
                
                let out = (*self.model)(
                    self.ts,
                    self.ts_1,
                    self.v_rest,
                    self.v_mem_old,
                    self.tao,
                    // è possibile fare unwrap() perchè altrimenti l'if sarebbe false
                    res_weighted_inputs.unwrap(),
                );
                self.ts_1 = self.ts;
                self.v_mem_old = out;
                println!("neuron [{}] emits {} at time [{}] --- threshold: {}" , self.name, out, self.ts, self.v_threshold);
                if out > self.v_threshold {
                    // se il modello fornisce un valore maggiore della soglia, resetta la tensione di membrana e assegna 1 all'out_spike
                    // e aggiorna ts_1 a ts
                    out_spike = 1;
                    self.v_mem_old = self.v_reset;
                } 
                
                //println!("{} out at {} : {}", self.to_string(), &self.ts, out_spike);   
            }
            // invia la spike a tutti i neuroni di output o al monitor
            // TODO: sarebbe meglio dare un return come Result 
            self.emit_spikes(out_spike);
            
            
            // attendi che gli altri thread facciano output prima di leggere gli input 
            barrier.wait();
            
        }
    }

    pub fn to_string(&self) -> String {
        return format!("[Neuron: {}]", self.name.clone());
    }
}
