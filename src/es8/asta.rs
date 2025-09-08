use std::sync::{mpsc, Arc, Mutex};
use crate::es8::banditore::Banditore;
use crate::es8::messaggio::Messaggio;
use crate::es8::partecipante::Partecipante;
use crate::es8::prodotto::Prodotto;
use crate::es8::risultato_asta::RisultatoAsta;

/** 
 * Struttura dati che rappresenta un'asta.
 * È composta da un banditore, un Vec di partecipanti e dal prodotto messo all'asta.
*/
pub struct Asta{
    banditore:Banditore,
    partecipanti:Vec<Partecipante>,
    prodotto:Prodotto,
}

impl Asta {
    
    /** 
     * Costruttore utile a creare una nuova asta. 
    */
    pub fn new(b:Banditore,v_p:Vec<Partecipante>,prod:Prodotto)->Self{
        Asta{banditore:b,partecipanti:v_p,prodotto:prod}
    }

    /** 
     * Metodo utile per ritornare un riferimento al Vec dei partecipanti.
    */
    pub fn get_partecipanti(&self)-> &Vec<Partecipante>{
        &self.partecipanti
    }

    /** 
     * Metodo utile per ritornare un riferimento al prodotto messo all'asta.
    */
    pub fn get_prodotto(&self)-> &Prodotto{
        &self.prodotto
    }

    /** 
     * Metodo utile per avviare l'asta.
     * Tale metodo crea un canale per la comunicazione dal partecipanti verso il banditore e 
     * altri 'n' canali con n pari al numero dei partecipanti per la comunicazione da banditore
     * verso partecipanti. Creo n canali perchè i Receveir non sono clonabili quindi necessito di più
     * canali per far arrivare a tutti i partecipanti i messaggi spediti dal banditore.
     * Dopo aver creato i vari canali avvio i thread corrispondenti ai partecipanti e al banditore.
    */
    pub fn avvia_asta(& mut self){

        // Canale da partecipanti a banditore
        let (tx_p,rx_b) = mpsc::channel::<Messaggio>();
        
        // Vec di tutti i sender usati dal banditore per inviare un messaggio
        let mut tx_band = Vec::new();
        // Vec di tutti i thread dei partecipanti
        let mut threads_partecipanti = Vec::new();

        let risultato = Arc::new(Mutex::new(RisultatoAsta::new(
                                                    None, 
                                                    self.get_prodotto().get_nome().clone(), 
                                                    None, 
                                                    self.get_prodotto().get_id(), 
                                                    self.get_prodotto().get_prezzo_partenza(), 
                                                    self.get_prodotto().get_prezzo_riserva())));

        for p in self.get_partecipanti(){
            // Canale da banditore a partecipante
            // Creo più canali per avere più receiver uno per ogni partecipante presente nell'asta
            let (tx_b,rx_p) = mpsc::channel::<Messaggio>();
            tx_band.push((tx_b,p.get_id()));

            // Dopo aver clonato il sender per il partecipante corrente avvierò anche il suo corrispondente Thread
            let tx_p_clone = tx_p.clone();
            let r = risultato.clone();

            let thread_partecipante = p.avvia_thread(tx_p_clone,rx_p,r);
            threads_partecipanti.push(thread_partecipante);
        }
        
        // Qui avvio il Thread del banditore a cui passo il prodotto, il vec con tutti i sender 
        // a cui scrivere un messaggio, il Receveir utile a ricevere i messaggi dei partecipanti e 
        // infine il numero di partecipanti all'asta
        let thread_banditore= self.banditore.avvia_thread(self.get_prodotto(), tx_band, rx_b,risultato.clone());
        thread_banditore.join().unwrap();
        
        for t in threads_partecipanti{
            let _ = t.join();
        }

    }

}