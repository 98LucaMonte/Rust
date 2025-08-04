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

    pub fn aggiorna_stato_conto(& mut self){

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
