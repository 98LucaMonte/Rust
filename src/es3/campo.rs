use std::fmt;
use rand::Rng;
use super::cella::Cella; 
use super::direzione::Direzione;
use super::livello::Livello;
/** 
 * Questa è una struct che descrive un Campo di gioco.
*/
#[derive(Clone)]
pub struct Campo{
    mappa: Vec<Vec<Cella>>,
}

/** 
 * Questo è il trait Display utile per avere una visualizzazione del Campo da gioco.
*/
impl fmt::Display for Campo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl Campo {

    /**
     * Metodo utile per accedere alla mappa della struct Campo.
     */
    pub fn get_campo(&self)-> Vec<Vec<Cella>>{
        self.mappa.clone()
    }

    /** 
     * Questa è l'implementazione del costruttore del campo da gioco.
     * Sono necessarie la dimensione del campo e il Livello con cui si vuole giocare la partita.
     * Inizialmente vengono inserite le Celle che corrispondono al Muro ai bordi del campo da gioco.
     * In base al Livello verranno create un certo numero di Celle velonose e di cibo che andranno 
     * rispettivamente a togliere e ad aggiungere forza al giocatore durante la partita.
     * La quantità di veleno e di cibo dipende sempre dal Livello. 
     * Le celle di veleno e cibo vengono inserite nel campo da gioco in modo randomico.
    */
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

    /**
     * Questo è il metodo utile per controllare se il giocatore si trova su un muro.
     * Vengono passati un riferimento al campo, la direzione attuale del giocatore 
     * e la posizione in cui si trova attualmente il giocatore.
     * Se la cella attuale e diversa da un muro restituisco la posizione e la direzione attuale,
     * altrimenti indico che è stato colpito il muro e restitutisco la direzione invertita e la nuova posizione
     * del giocatore sarà opposta a quella intrapresa in precedenza.
     */
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

    /**
     * Questo è il metodo utile per cambiare il valore della cella che si è appena visitata o che si sta visitando.
     * Si prende un riferimento del campo e la posizione attuale e la posizione precedente del giocatore.
     * Con la posizione attuale del giocatore verrà indicato che quella è la posizione attuale del giocatore, mentre
     * con la posizione precedente del giocatore viene inserita una cella vuota poiché quello che vi era soprà
     * è stato "mangiato" dal giocatore.
     */
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
