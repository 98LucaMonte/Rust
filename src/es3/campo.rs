
  
use std::fmt;
use rand::Rng;
use super::cella::Cella; 
use super::direzione::Direzione;
use super::livello::Livello;

#[derive(Clone)]
pub struct Campo{
    pub mappa: Vec<Vec<Cella>>,
}

impl fmt::Display for Campo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl Campo {

    pub fn new(dimensione: usize, livello: &Livello)-> Campo{
        let mut mappa: Vec<Vec<Cella>> = vec![vec![Cella::Vuota(0);dimensione];dimensione];

        let (_forza,mut cibo,mut veleno,_mosse) = livello.scelta_livello(dimensione as u32);

        for riga in 0..dimensione{
            for colonna in 0..dimensione{
                if riga == 0 || colonna == 0 || riga == dimensione-1 || colonna == dimensione-1{
                    mappa[riga][colonna] = Cella::Muro;
                } 
            }
        } 

        while cibo != 0{
            let riga = rand::thread_rng().gen_range(1..dimensione);
            let colonna = rand::thread_rng().gen_range(1..dimensione);
            if mappa[riga][colonna] == Cella::Vuota(0) && mappa[riga][colonna] != Cella::Muro{
                match livello {
                    Livello::Facile |Livello::Medio => mappa[riga][colonna] = Cella::Cibo(2),
                    Livello::Difficile => mappa[riga][colonna] = Cella::Cibo(1),
                }
                cibo -= 1;
            }  

        }

        while veleno != 0{
            let riga = rand::thread_rng().gen_range(1..dimensione);
            let colonna = rand::thread_rng().gen_range(1..dimensione);
            if mappa[riga][colonna] == Cella::Vuota(0) && mappa[riga][colonna] != Cella::Muro && mappa[riga][colonna] != Cella::Cibo(2) {
                match livello {
                    Livello::Facile => mappa[riga][colonna] = Cella::Veleno(-2),
                    Livello::Medio => mappa[riga][colonna] = Cella::Veleno(-3),
                    Livello::Difficile => mappa[riga][colonna] = Cella::Veleno(-4),
                }
                
                veleno -= 1;
            }  
        }

        Campo { mappa: mappa }
    }

    pub fn controllo_muro(& self, dir: Direzione,pos: (usize,usize))-> (Direzione,usize,usize){

        match self.mappa[pos.0][pos.1]{
            Cella::Vuota(_) | Cella::Cibo(_) | Cella::Veleno(_) | Cella::Player => (dir,pos.0,pos.1),
            Cella::Muro => {

                match dir{
                    Direzione::Su => {
                        println!("Abbiamo colpito il muro, andiamo nella direzione opposta.\nDirezione attuale: {}", Direzione::Giu);
                        (Direzione::Giu,pos.0 + 2,pos.1)
                    },
                    Direzione::Giu => {
                        println!("Abbiamo colpito il muro, andiamo nella direzione opposta.\nDirezione attuale: {}", Direzione::Su);
                        (Direzione::Su,pos.0 - 2,pos.1)
                    },
                    Direzione::Destra => {
                        println!("Abbiamo colpito il muro, andiamo nella direzione opposta.\nDirezione attuale: {}", Direzione::Sinistra);
                        (Direzione::Sinistra,pos.0,pos.1 - 2)
                    },
                    Direzione::Sinistra => {
                        println!("Abbiamo colpito il muro, andiamo nella direzione opposta.\nDirezione attuale: {}", Direzione::Destra);
                        (Direzione::Destra,pos.0,pos.1 + 2)
                    },
                }

            },
        }
        


    }

    pub fn cambia_valore_cella(& mut self, pos: (usize,usize),pos_prec: (usize,usize)){

        match self.mappa[pos.0][pos.1]{
            Cella::Muro | Cella::Player => (),
            Cella::Cibo(_) | Cella::Veleno(_) | Cella::Vuota(_) => {
                self.mappa[pos.0][pos.1] = Cella::Player;
            },
        }

        match self.mappa[pos_prec.0][pos_prec.1]{
            Cella::Vuota(_) | Cella::Muro | Cella::Cibo(_) | Cella::Veleno(_) => (),
            Cella::Player => {
                self.mappa[pos_prec.0][pos_prec.1] = Cella::Vuota(0);
            },
        }
    }
}
