use crate::es5::stato::{Argento, Oro, Rosso, Stato};   
/** 
 * Questa è una struct che rappresenta un conto bancario.
*/
pub struct ContoBancario{
    nome_cliente: String,
    saldo: f32,
    limite_inferiore: f32,
    limite_superiore: f32,
    interesse: f32,
    stato: Box<dyn Stato>,
}

impl ContoBancario{

    /** 
     * Costruttore utile a creare un nuovo conto bancario.
     * Se il limite inferiore è maggiore del limite superiore panica.
     * Se il saldo è inferiore del limite inferiore il conto è in stato Rosso,
     * se il saldo è maggiore del limite superiore il conto è in stato Oro,
     * altrimenti il conto è in stato Argento.
    */
    pub fn new(nome_cliente:String,saldo:f32,limite_inferiore:f32,limite_superiore:f32,interesse:f32)-> ContoBancario{
        
        // controllo il limite inferiore deve essere minore del limite superiore
        if limite_inferiore > limite_superiore{
            panic!("Il limite superiore deve essere maggiore del limite inferiore");
        }

        let stato: Box<dyn Stato>;

        if saldo < limite_inferiore{
            stato = Box::new(Rosso)
        }
        else if saldo > limite_superiore {
            stato = Box::new(Oro)
        }
        else {
            stato = Box::new(Argento)
        }

        ContoBancario { nome_cliente: nome_cliente, saldo: saldo, limite_inferiore: limite_inferiore, 
                        limite_superiore: limite_superiore, interesse: interesse, stato: stato }
    }

    /** 
     * Metodo che richiama il metodo deposita contenuto nel trait Stato (e verra richiamato nella struct in cui appartiene Stato ossia Rosso,Argento o Oro). 
     * Dopo aver eseguito il metodo si aggiorna lo stato del conto.
    */
    pub fn deposita(& mut self,quantita:f32){
        // facendo deposita su stato vado ad eseguire deposita dallo stato attuale del conto bancario
        self.stato.deposita(& mut self.saldo,quantita);
        self.aggiorna_stato_conto();
    }

    /** 
     * Metodo che richiama il metodo preleva contenuto nel trait Stato (e verra richiamato nella struct in cui appartiene Stato ossia Rosso,Argento o Oro). 
     * Eseguo un controllo per verificare se il saldo sia effettivamente maggiore della quantità da prelevare.
     * Dopo aver eseguito il metodo si aggiorna lo stato del conto.
    */
    pub fn preleva(& mut self,quantita:f32){
        // facendo preleva su stato vado ad eseguire preleva dallo stato attuale del conto bancario
        // si esegue preleva soltanto se il saldo attuale è maggiore uguale alla quantità richiesta da prelevare.
        if self.saldo >= quantita {
            self.stato.preleva(& mut self.saldo,quantita);
            self.aggiorna_stato_conto();
        }
    }

    /** 
     * Metodo che richiama il metodo paga_interessi contenuto nel trait Stato (e verra richiamato nella struct in cui appartiene Stato ossia Rosso,Argento o Oro). 
     * Dopo aver eseguito il metodo si aggiorna lo stato del conto.
    */
    pub fn paga_interessi(& mut self){
        // facendo paga_interessi su stato vado ad eseguire paga_interessi dallo stato attuale del conto bancario
        self.stato.paga_interessi(& mut self.saldo,&self.interesse);
        self.aggiorna_stato_conto(); 
    }

    /** 
     * Metodo che aggiorna lo stato del conto bancario in base al suo saldo e i valori di limiti inferiori e superiori.
     * Se il limite inferiore è maggiore del limite superiore panica.
     * Se il saldo è inferiore del limite inferiore il conto è in stato Rosso,
     * se il saldo è maggiore del limite superiore il conto è in stato Oro,
     * altrimenti il conto è in stato Argento.
    */
    fn aggiorna_stato_conto(& mut self){

        if self.saldo < self.limite_inferiore{
            self.stato = Box::new(Rosso); 
        }
        else if self.saldo > self.limite_superiore {
            self.stato = Box::new(Oro); 
        }
        else {
            self.stato = Box::new(Argento); 
        }

    }

    /**
     * Metodo utile per accedere al saldo del conto_bancario.
     */
    pub fn get_saldo(&self)->f32{
        self.saldo
    }

    /**
     * Metodo utile per accedere al nome del cliente del conto_bancario.
     */
    pub fn get_nome_cliente(&self)->String{
        self.nome_cliente.clone()
    }

    /** 
     * Metodo utile per accedere allo stato del conto bancario.
    */
    pub fn get_stato(&self) -> &Box<dyn Stato>{
        &self.stato
    }
}
