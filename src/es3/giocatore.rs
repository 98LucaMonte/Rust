use std::fmt;
use rand::Rng;
use super::cella::Cella; 
use super::campo::Campo; 
use super::direzione::Direzione;
use super::livello::Livello;
  
pub struct Giocatore{
    pub posizione: (usize,usize),
    pub direzione: Direzione,
    pub forza: i32,
    pub mosse: u32,
}

impl fmt::Display for Giocatore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl Giocatore {

    pub fn new(campo: &Campo, livello: &Livello)-> Giocatore{
        
        let pos: (usize,usize);
        let dir: Direzione;
        let dimensione = campo.mappa.len();
        let (forza,_cibo,_veleno,mosse) = livello.scelta_livello(dimensione as u32);

        loop{
            let riga = rand::thread_rng().gen_range(1..dimensione);
            let colonna = rand::thread_rng().gen_range(1..dimensione);
            if campo.mappa[riga][colonna] == Cella::Vuota(0){
                pos = (riga,colonna);
                break;
            }
        }

        let n = rand::thread_rng().gen_range(0..=3);

        match n {
            0 => {
                dir = Direzione::Su;
            },
            1 => {
                dir = Direzione::Giu;
            },
            2 => {
                dir = Direzione::Destra;
            },
            _ => {
                dir = Direzione::Sinistra;
            },
        } 
        
        Giocatore{posizione:pos,direzione:dir,forza: forza, mosse: mosse}
    }

    pub fn muovi_giocatore(& mut self, campo: &Campo){

        let direzione: Direzione = self.direzione.clone();
        let dir;
        let (pos0, pos1);

        match direzione {
            Direzione::Su => {
                (dir, pos0,pos1) = campo.controllo_muro(direzione, (self.posizione.0-1,self.posizione.1));
            },
            Direzione::Giu => {
                (dir, pos0,pos1) = campo.controllo_muro(direzione, (self.posizione.0+1,self.posizione.1));
            },
            Direzione::Destra => {
                (dir, pos0,pos1) = campo.controllo_muro(direzione, (self.posizione.0,self.posizione.1+1));
            },
            Direzione::Sinistra => {
                (dir, pos0,pos1) = campo.controllo_muro(direzione, (self.posizione.0,self.posizione.1-1));
            },
        }

        self.posizione = (pos0,pos1);
        self.direzione = dir;            

    }

    pub fn modifica_forza_giocatore(& mut self,campo: & Campo){

        let posizione = self.posizione;
        let cella_corrente = campo.mappa.clone();

        match cella_corrente[posizione.0][posizione.1] {
            // Per Veleno val è negativo per Cibo sarà positivo
            Cella::Cibo(val) | Cella::Veleno(val) => {
                self.forza += val;
            },
            Cella::Vuota(0) | Cella::Muro | _ => (),
        }

    }

}