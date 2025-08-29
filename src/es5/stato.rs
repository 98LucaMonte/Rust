/** 
 * Implementazione del Trait Stato utile a rappresentare e poi ad implementare il pattern State
 * Tale Trait implementa i metodi:
 * - deposita che aumentano il saldo di un conto bancario in base al valore di quantita,
 * - preleva che diminuisce il saldo di un conto bancario in base al valore di quantita,
 * - paga_interessi che aumenta il saldo di un conto bancario in base agli interessi di quel conto,
 * - stato_attuale_conto che restitusce un enum che rappresenta lo stato attuale del conto.
*/
pub trait Stato{
    fn deposita(&self, saldo: & mut f32, quantita:f32){
        *saldo += quantita;
    }
    fn preleva(&self, saldo: & mut f32, quantita:f32);
    fn paga_interessi(&self, saldo: & mut f32, interesse: &f32);
    fn stato_attuale_conto(&self) -> &str;
}

/** 
 * Implementazione della struct Oro che implementa il Trait Stato. Implementa tutti i metodi di Stato.
*/
pub struct Oro;  
impl Stato for Oro {
    
    fn preleva(&self, saldo: & mut f32, quantita:f32) {
        *saldo -= quantita;
    }
    
    fn paga_interessi(&self, saldo: & mut f32, interesse: &f32) {
        *saldo += *saldo * *interesse;
    }

    fn stato_attuale_conto(&self) -> &str {"Oro"}

}

/** 
 * Implementazione della struct Argento che implementa il Trait Stato. Implementa tutti i metodi di Stato tranne il paga_interessi.
*/
pub struct Argento;
impl Stato for Argento{
        
    fn preleva(&self, saldo: & mut f32, quantita:f32) {
        *saldo -= quantita;
    }

    fn paga_interessi(&self, _saldo: & mut f32, _interesse: &f32) {
        
    }

    fn stato_attuale_conto(&self)->&str {"Argento"}

}

/** 
 * Implementazione della struct Argento che implementa il Trait Stato. Implementa solo il metodo deposita e stato_attuale_conto.
*/
pub struct Rosso;
impl Stato for Rosso {

    fn preleva(&self, _saldo: & mut f32, _quantita:f32) {
        
    }

    fn paga_interessi(&self, _saldo: & mut f32, _interesse: &f32) {
        
    }

    fn stato_attuale_conto(&self)->&str {"Rosso"}
}