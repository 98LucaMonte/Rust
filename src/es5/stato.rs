pub struct Oro;  
pub struct Argento;
pub struct Rosso;

#[derive(PartialEq,Debug)]
pub enum StatoTipo{
    Oro,Argento,Rosso,
}

pub trait Stato{
    fn deposita(&self, saldo: & mut f32, quantita:f32);
    fn preleva(&self, saldo: & mut f32, quantita:f32);
    fn paga_interessi(&self, saldo: & mut f32, interesse: &f32);
    fn stato_attuale_conto(&self) -> StatoTipo;
}

impl Stato for Oro {
    fn deposita(&self, saldo: & mut f32, quantita:f32) {
        *saldo += quantita;
    }

    fn preleva(&self, saldo: & mut f32, quantita:f32) {
        *saldo -= quantita;
    }

    fn paga_interessi(&self, saldo: & mut f32, interesse: &f32) {
        *saldo += *saldo * *interesse;
    }

    fn stato_attuale_conto(&self) -> StatoTipo {
        StatoTipo::Oro
    }

}

impl Stato for Argento{
    fn deposita(&self, saldo: & mut f32, quantita:f32) {
        *saldo += quantita;
    }

    fn preleva(&self, saldo: & mut f32, quantita:f32) {
        *saldo -= quantita;
    }

    fn paga_interessi(&self, _saldo: & mut f32,_interesse: &f32) { 
    }

    fn stato_attuale_conto(&self)->StatoTipo {
        StatoTipo::Argento
    }

}

impl Stato for Rosso {
    fn deposita(&self, saldo: & mut f32, quantita:f32) {
        *saldo += quantita;
    }

    fn preleva(&self, _saldo: & mut f32, _quantita:f32) {
    }

    fn paga_interessi(&self, _saldo: & mut f32,_interesse: &f32) {
    }

    fn stato_attuale_conto(&self)->StatoTipo {
        StatoTipo::Rosso
    }
}