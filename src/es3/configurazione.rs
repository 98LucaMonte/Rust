


use std::fmt;
use rand::Rng;
use super::campo::Campo; 
use super::giocatore::Giocatore;
use super::moneta::Moneta;
use super::direzione::Direzione;

pub struct Configurazione{
    pub campo : Campo,
    pub giocatore : Giocatore,
}

impl fmt::Display for Configurazione {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"-----------------------------------------------------------------------------------")?;
        writeln!(f,"\nDati del giocatore:\t Posizione: ({},{}),\t Direzione: {},\t Forza: {}, \t Mosse: {}.",
                self.giocatore.posizione.0,self.giocatore.posizione.1,self.giocatore.direzione,self.giocatore.forza,self.giocatore.mosse)?;

        writeln!(f,"Situazione attuale del campo \n")?;            
        for num_col in 0..self.campo.mappa.len(){
            write!(f, "\t {}",num_col)?;
        }

        for (num_riga,riga) in self.campo.mappa.iter().enumerate(){
            write!(f,"\n{}",num_riga)?;
            for cella in riga{
                write!(f,"\t{}", cella)?;
            }
        }
        write!(f,"")
    }
} 

impl Configurazione { 
    
    pub fn new(campo: Campo, giocatore: Giocatore) -> Configurazione{
        Configurazione {campo: campo, giocatore: giocatore}
    }

    pub fn avvio_partita(& mut self){

        self.campo.cambia_valore_cella(self.giocatore.posizione, (0,0));

        while self.giocatore.forza > 0 && self.giocatore.mosse != 0 {
            
            println!(" {} ",self);

            let moneta: Moneta = Moneta::lancio_moneta();
            let posizione_precedente = self.giocatore.posizione;
            
            match moneta {
                Moneta::Testa => {
                    println!("È uscita testa ci si sposta nella direzione indicata dal giocatore");
                },
                Moneta::Croce => {
                    println!("È uscita croce quindi si genera una direzione casuale in cui muoversi");
                    
                    let mut arr_direzioni = vec![Direzione::Su,Direzione::Giu,Direzione::Sinistra,Direzione::Destra];

                    arr_direzioni.retain(|x| *x != self.giocatore.direzione);

                    let n = rand::thread_rng().gen_range(0..arr_direzioni.len());
                        
                    self.giocatore.direzione = arr_direzioni[n].clone();
                    
                    println!("Il giocatore si muove a {}",self.giocatore.direzione);
                },
            }

            self.giocatore.muovi_giocatore(& self.campo);
            self.giocatore.modifica_forza_giocatore(& self.campo);
            self.giocatore.mosse = self.giocatore.mosse - 1;
            self.campo.cambia_valore_cella(self.giocatore.posizione,posizione_precedente);       
        } 

        if self.giocatore.forza > 0 && self.giocatore.mosse == 0 {
            println!("Complimenti hai vinto");
        }
        else {
            println!("Mi dispiace hai perso");
        }
        
    }
}


