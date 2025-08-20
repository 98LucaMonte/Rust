use std::fmt;
use rand::Rng;
use super::campo::Campo; 
use super::giocatore::Giocatore;
use super::moneta::Moneta;
use super::direzione::Direzione;

/** 
 * Questa è una struct che descrive una Configurazione quindi come sarà strutturata una partita del gioco.
*/
pub struct Configurazione{
    campo : Campo,
    giocatore : Giocatore,
}

/** 
 * Questo è il trait Display utile per avere una visualizzazione della Configurazione.
 * A differenza di Giocatore e di Campo qui vengono stampate per ogni turno di gioco:
 * - Le informazioni riguardanti il Giocatore in quell'istante,
 * - Le informazioni riguardanti il Campo da gioco in quell'istante. 
*/
impl fmt::Display for Configurazione {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"-----------------------------------------------------------------------------------")?;
        writeln!(f,"\nDati del giocatore:\t Posizione: ({},{}),\t Direzione: {},\t Forza: {}, \t Mosse: {}.",
                self.giocatore.get_posizione().0,self.giocatore.get_posizione().1,self.giocatore.get_direzione(),self.giocatore.get_forza(),self.giocatore.get_mosse())?;

        writeln!(f,"Situazione attuale del campo \n")?;            
        for num_col in 0..self.campo.get_campo().len(){
            write!(f, "\t {}",num_col)?;
        }

        for (num_riga,riga) in self.campo.get_campo().iter().enumerate(){
            write!(f,"\n{}",num_riga)?;
            for cella in riga{
                write!(f,"\t{}", cella)?;
            }
        }
        write!(f,"")
    }
} 

impl Configurazione { 
    
    /** 
     * Questa è l'implementazione del costruttore della configurazione.
    */
    pub fn new(campo: Campo, giocatore: Giocatore) -> Configurazione{
        Configurazione {campo: campo, giocatore: giocatore}
    }

    /**
     * Questo è il metodo utile ad avviare la partita.
     * Per prima cosa viene richiamata cambia_valore_cella di Campo per indicare dove si trova all'inizio il giocatore e inoltre,
     * come secondo parametro di quel metodo viene passato (0,0) in tal modo siamo sicuri che è un muro e non succede nulla al campo 
     * da Gioco (vedi implementazione di quel metodo).
     * Il gioco finisce quando la forza del Giocatore è minore uguale a 0 e in tal caso abbiamo perso oppure quando sono finire il numero di mosse.
     * Se sono finite le mosse e la forza è positiva abbiamo vinto la partita.
     * Ogni turno viene lanciata la moneta e indichiamo all'utente che cosa è uscito. Per testa ci si muove nella direzione indicata dall'utente,
     * altrimenti se esce croce ci si muove in modo randomico in una direzione diversa da quella attuale dell'utente. 
     */
    pub fn avvio_partita(& mut self){

        self.campo.cambia_valore_cella(self.giocatore.get_posizione(), (0,0));

        while self.giocatore.get_forza() > 0 && self.giocatore.get_mosse() != 0 {
            
            println!(" {} ",self);

            let moneta: Moneta = Moneta::lancio_moneta();
            let posizione_precedente = self.giocatore.get_posizione();
            
            match moneta {
                Moneta::Testa => {
                    println!("È uscita testa ci si sposta nella direzione indicata dal giocatore");
                },
                Moneta::Croce => {
                    println!("È uscita croce quindi si genera una direzione casuale in cui muoversi");
                    
                    /*
                     * In questo caso creo un Vettore di Direzione in cui sono presenti tutte le direzioni
                     * dopodiché mantengo tutte le direzioni diverse da quella attuale del giocatore.
                     * Infine genero un numero tra 0 e la (lunghezza del vettore)-1 e in questo modo ho la nuova 
                     * direzione del giocatore scelta in modo randomico. 
                    */
                    let mut arr_direzioni = vec![Direzione::Su,Direzione::Giu,Direzione::Sinistra,Direzione::Destra];
                    arr_direzioni.retain(|x| *x != self.giocatore.get_direzione());
                    let n = rand::thread_rng().gen_range(0..arr_direzioni.len());
                    self.giocatore.set_direzione(arr_direzioni[n].clone()); 
                    
                    println!("Il giocatore si muove a {}",self.giocatore.get_direzione());
                },
            }

            self.giocatore.muovi_giocatore(& self.campo);
            self.giocatore.modifica_forza_giocatore(& self.campo);
            self.giocatore.set_mosse();
            self.campo.cambia_valore_cella(self.giocatore.get_posizione(),posizione_precedente);       
        } 

        if self.giocatore.get_forza() > 0 && self.giocatore.get_mosse() == 0 {
            println!("Complimenti hai vinto");
        }
        else {
            println!("Mi dispiace hai perso");
        }
        
    }
}


