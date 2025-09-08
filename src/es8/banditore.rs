use std::{sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, thread::{self}};
use crate::es8::{messaggio::Messaggio, prodotto::Prodotto, risultato_asta::RisultatoAsta};

/** 
 * Struttura che rapprensenta il Banditore dell'asta.
*/
pub struct Banditore{
    nome: String
}

impl Banditore {
    
    /** 
     * Costruttore utile a creare un nuovo Banditore. 
    */
    pub fn new(nome:&str)->Self{
        Banditore { nome: nome.to_string() }
    }

    /** 
     * Metodo utile per ritornare il nome del Banditore.
    */
    pub fn get_nome(&self)->&String{
        &self.nome
    }

    /** 
     * Metodo utile ad avviare il comportamento del Banditore all'interno dell'asta.
    */

    /*
     * Metodo utile per avviare il Thread che determinarà il comportamento del Banditore all'asta.
     * Per il Banditore ci sarà un vec di tuple composte da un Sender e l'id del partecipante associato a tale Sender con cui mandare messaggi 
     * verso quello specifico Partecipante e un Receiver con cui ricevere messaggi dai partecipanti.
     * Inizialmente il banditore attende l'arrivo del messaggio proveniente dai partecipanti riguardo l'ingresso nell'asta.
     * Attende che tutti entrino nell'asta e poi invia le informazioni sul prodotto.
     * Dopodiché, attende messaggi di offerta e di esci dall'asta.
     * Se arriva un offerta si verifica se quest'offerta sia maggiore del prezzo raggiunto finora dall'asta e se è così si aggiorna 
     * il risultato dell'asta con il nome e l'id del possibile vincitore e l'offerta che ha fatto e si inviano le informazioni sul prezzo 
     * raggiunto finora dall'asta solo ai partecipanti attivi ossia che non sono ancora usciti dall'asta.
     * Se arriva un uscita dall'asta allora si aggiorna il vettore di partecipanti attivi all'asta e quando ne rimane solo un 
     * partecipante attivo allora si invia il messaggio di fine asta ai partecipanti informandoli su come sia andata.
     * Fine comportamento Banditore.
     */
    pub fn avvia_thread(&self,prodotto:&Prodotto,vec_tx_id:Vec<(Sender<Messaggio>, usize)>,rx:Receiver<Messaggio>,risultato:Arc<Mutex<RisultatoAsta>>) -> thread::JoinHandle<()>{

        let nome_banditore = self.get_nome().clone();
        let mut qty = 0;

        let descrizione_prodotto = prodotto.get_descrizione().clone();
        let prezzo_iniziale = prodotto.get_prezzo_partenza();

        let mut vec_tx_id_attivi = vec_tx_id.clone();
        
        let thread_banditore = thread::spawn(move || {
            
            while let Ok(msg) = rx.recv() {

                match msg {
                    Messaggio::Partecipo { id, nome } => {

                        // Ricevo il messaggio di partecipazione all'asta.
                        println!("[Banditore {}] Il partecipante {} con id ({}) si è unito all'asta.",nome_banditore,nome,id);
                        qty += 1;
                        
                        // Il messaggio è arrivato da tutti i partecipanti posso quindi inviare le informazioni sull'Asta.
                        if qty == vec_tx_id.len(){
                            println!("\n\n[Banditore {}] Si dia inizio all'asta.\n\n",nome_banditore);
                            for (tx,_id_partecipante) in &vec_tx_id{
                                let _ = tx.send(Messaggio::Informazioni { descrizione_prodotto: descrizione_prodotto.clone(), prezzo_attuale: prezzo_iniziale });
                            }
                        }

                    },
                    Messaggio::Offerta { nome_partecipante, id_partecipante, prezzo } => {

                        {
                            let mut r = risultato.lock().unwrap();
                            
                            if prezzo > r.get_prezzo_venduto(){
                                // Se l'offerta ricevuta è superiore a quella attuale che vince l'asta allora aggiorno il prezzo, l'id e il nome del partecipante che sta vincendo l'asta.
                                r.set_prezzo_venduto(prezzo);
                                r.set_nome_partecipante(nome_partecipante.clone());
                                r.set_id_partecipante(id_partecipante);
                                
                                println!("[Banditore {}] Il partecipante {} con id ({}) ha fatto un'offerta di {}",nome_banditore,nome_partecipante,id_partecipante,prezzo);
                                
                                for (tx,id) in &vec_tx_id_attivi{
                                    // Invio la nuova informazione a tutti i partecipanti ancora presenti nell'asta tranne a quello che ha fatto l'offerta
                                    if *id != id_partecipante {
                                        let _ = tx.send(Messaggio::Informazioni { descrizione_prodotto: descrizione_prodotto.clone(), prezzo_attuale: prezzo });
                                    }
                                }

                            }

                        }
                       
                    },
                    Messaggio::Esci { nome_partecipante, id_partecipante } => {

                        println!("[Banditore {}] Il partecipante {} con id({}) è uscito dall'asta.",nome_banditore,nome_partecipante,id_partecipante);
                        vec_tx_id_attivi.retain(|x| x.1 != id_partecipante);

                        if vec_tx_id_attivi.len() == 1 {
                            // Invio il messaggio di fine asta a tutti i partecipanti
                            for (tx,_id_partecipante) in &vec_tx_id{
                                let _ = tx.send(Messaggio::FineAsta);
                            }
                            break;
                        }

                    },
                    _ => (),
                }
            }

        });
        thread_banditore
    }
}