# snn
Spike Neural Network project 

### TODO
- creare una classe Ouput e sostituire tutti i Receiver e i SynchSender di i8 con Receiver\SynchSender<Output>, questa classe conterrà i campi from e spike 
dove from indica il neurone del layer di provenienza e spike è un i8, modificare le varie receive e emit spike in accordo a queste modifiche con delle get_spike
se si volgiono mantenere gli attributi privati o con .spike se si decide di mantenerli pubblici.
- terminare la classe Output Monitor aggiungendo la stampa su file, ragionando sul formato una soluzione potrebbe essere  \
    ts  |   n0  | ... | n_n \
    0       0      1      1 \
    1       0      0      0 \
    2       1      0      0 \
    ....
 e creare un metodo che conti le spike a 1 per avere un altro tipo di output. 
- scrivere alcuni semplici test per testare le connessioni e controllare che non ci siano deadlock
- gestione degli errori e dei valori di ritorno sotto forma di result: creare un file Error in cui si specifica il tipo di errore contenente la stringa di descrizione
( prendere spunto dal lavoro dell'altro gruppo se si vuole )
- scrivere un factory method per creare l'intera rete da file (magari JSON) perchè per reti molto grandi è scomodo chiamare continuamente la connect, aggiungere 
magari un metodo di questo tipo per i layer in modo da modularizzare la soluzione 
- creare un modulo per gli errori e sostituire le varie `panic!` con delle `Result`, pensare ai possibili errori come nelle varie connessioni, letture dei file, 
- ci sono varie *TODO* nel codice di pezzi che mancano  
