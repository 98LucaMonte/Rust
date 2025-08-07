use crate::es5::stato::{Argento, Oro, Rosso, Stato};   
pub struct ContoBancario{
    pub nome_cliente: String,
    pub saldo: f32,
    pub limite_inferiore: f32,
    pub limite_superiore: f32,
    pub interesse: f32,
    pub stato: Box<dyn Stato>,
}

impl ContoBancario{

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

    pub fn deposita(& mut self,quantita:f32){
        // facendo deposita su stato vado ad eseguire deposita dallo stato attuale del conto bancario
        self.stato.deposita(& mut self.saldo,quantita);
        self.aggiorna_stato_conto();
    }

    pub fn preleva(& mut self,quantita:f32){
        // facendo preleva su stato vado ad eseguire deposita dallo stato attuale del conto bancario
        self.stato.preleva(& mut self.saldo,quantita);
        self.aggiorna_stato_conto();
    }

    pub fn paga_interessi(& mut self){
        // facendo paga_interessi su stato vado ad eseguire deposita dallo stato attuale del conto bancario
        self.stato.paga_interessi(& mut self.saldo,&self.interesse);
        self.aggiorna_stato_conto();
    }

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

}
