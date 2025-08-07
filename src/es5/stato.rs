pub trait Stato{
    fn deposita(&self, saldo: & mut f32, quantita:f32){
        *saldo += quantita;
    }
    fn preleva(&self, saldo: & mut f32, quantita:f32) {
        *saldo -= quantita;
    }
    fn paga_interessi(&self, saldo: & mut f32, interesse: &f32);
    fn stato_attuale_conto(&self) -> StatoTipo;
}

pub struct Oro;  
impl Stato for Oro {
    
    fn paga_interessi(&self, saldo: & mut f32, interesse: &f32) {
        *saldo += *saldo * *interesse;
    }

    fn stato_attuale_conto(&self) -> StatoTipo {StatoTipo::Oro}

}

pub struct Argento;
impl Stato for Argento{
    
    fn paga_interessi(&self, _saldo: & mut f32,_interesse: &f32) { }

    fn stato_attuale_conto(&self)->StatoTipo {StatoTipo::Argento}

}

pub struct Rosso;
impl Stato for Rosso {

    fn preleva(&self, _saldo: & mut f32, _quantita:f32) {}

    fn paga_interessi(&self, _saldo: & mut f32,_interesse: &f32) {}

    fn stato_attuale_conto(&self)->StatoTipo {StatoTipo::Rosso}
}

#[derive(PartialEq,Debug)]
pub enum StatoTipo{
    Oro,Argento,Rosso,
}