use std::fmt;
use rand::Rng;
use super::cella::Cella; 
use super::campo::Campo; 
use super::direzione::Direzione;
use super::livello::Livello;

/** 
 * Questa è una struct che descrive un Giocatore.
*/
pub struct Giocatore{
    posizione: (usize,usize),
    direzione: Direzione,
    forza: i32,
    mosse: u32,
}

/** 
 * Questo è il trait Display utile per avere una visualizzazione del Giocatore.
*/
impl fmt::Display for Giocatore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl Giocatore {

    /**
     * Metodo utile per accedere alla posizione del giocatore.
     */
    pub fn get_posizione(&self)-> (usize,usize){
        (self.posizione.0,self.posizione.1)
    }

    /**
     * Metodo utile per accedere alla direzione del giocatore.
     */
    pub fn get_direzione(&self)-> Direzione{
        self.direzione.clone()
    }

    /**
     * Metodo utile per accedere alla forza del giocatore.
     */
    pub fn get_forza(&self)->i32{
        self.forza
    }

    /**
     * Metodo utile per accedere alle mosse del giocatore.
     */
    pub fn get_mosse(&self)->u32{
        self.mosse
    }

    /**
     * Metodo utile per settare una nuova direzione del giocatore.
     */
    pub fn set_direzione(& mut self, dir:Direzione){
        self.direzione = dir;
    }

    /**
     * Metodo utile per settare il numero di mosse mancanti al giocatore.
     */
    pub fn set_mosse(& mut self){
        self.mosse -= 1;
    }

    /**
     * Metodo utile per settare la posizione del giocatore.
     */
    pub fn set_posizione(& mut self, pos:(usize,usize)){
        self.posizione = pos;
    }

    /**
     * Costruttore del Giocatore. Occorrono sia il Livello sia il campo da gioco.
     * Il campo da gioco serve per individuare una posizione casuale (che sia una cella vuota)
     * in cui posizionare il giocatore.
     * Il livello invece serve per ricevere le informazioni su quanta forza e su quante mosse 
     * avrà il giocatore in questa partita.
     * Inoltre, viene generato in modo stocastico quale sarà la direzione iniziale del giocatore.
     */
    pub fn new(campo: &Campo, livello: &Livello)-> Giocatore{
        
        let pos: (usize,usize);
        let dir: Direzione;
        let dimensione = campo.get_campo().len();
        let (forza,_cibo,_veleno,mosse) = livello.scelta_livello(dimensione as u32);

        loop{
            let riga = rand::thread_rng().gen_range(1..dimensione);
            let colonna = rand::thread_rng().gen_range(1..dimensione);
            if campo.get_campo()[riga][colonna] == Cella::Vuota(0){
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

    /**
     * Questo metodo è utile per muovere il giocatore all'interno del campo da gioco.
     * All'interno del metodo viene fatto un controllo. Se la posizione attuale ossia se la cella attuale
     * è un muro allora vengono restituiti dal metodo controllo muro la direzione opposta rispetto 
     * a quella attuale e la posizione in cui ci si ritroverebbe nel caso in cui ci fossimo spostati
     * proprio nella direzione opposta.
     */
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

    /**
     * Questo metodo è utile per modificare la forza del giocatore in base 
     * alla cella del campo in cui ci si ritrova. 
     * Se la cella è vuota o è un muro non si modifica la forza altrimenti se siamo 
     * in una cella velenosa o con cibo la si andrà rispettivamente ad aumentare o diminuire.
     */
    pub fn modifica_forza_giocatore(& mut self,campo: & Campo){

        let posizione = self.posizione;
        let cella_corrente = campo.get_campo().clone();

        match cella_corrente[posizione.0][posizione.1] {
            // Per Veleno val è negativo per Cibo sarà positivo
            Cella::Cibo(val) | Cella::Veleno(val) => {
                self.forza += val;
            },
            Cella::Vuota(0) | Cella::Muro | _ => (),
        }

    }

}