use std::{sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, thread};
use rand::Rng;
use crate::es8::{messaggio::Messaggio, moneta::Moneta, risultato_asta::RisultatoAsta};

/** 
 * Struttura che rapprensenta il Partecipante dell'asta.
 * Al suo interno si trova il nome, l'identificativo univoco di ogni partecipante e la sua disponibilità economica.
*/
pub struct Partecipante{
    nome: Option<String>,
    id: Option<usize>,
    disponibilità: usize,
}

impl Partecipante {
    
    /** 
     * Costruttore che utile a creare un nuovo Partecipante. 
    */
    pub fn new(id:usize)->Self{
        let (nome,disponibilita) = Partecipante::genera_nome_disponibilita();
        Partecipante { nome: Some(nome.to_string()),id:Some(id),disponibilità:disponibilita }
    }

    /** 
     * Metodo utile per ritornare il nome del Partecipante.
    */
    pub fn get_nome(&self)->&String{
        self.nome.as_ref().unwrap()
    }

    /** 
     * Metodo utile per ritornare l'identificativo del Partecipante.
    */
    pub fn get_id(&self)->usize{
        *self.id.as_ref().unwrap()
    }

    /** 
     * Metodo utile per ritornare la disponibilità economica del Partecipante.
    */
    pub fn get_disponibilita(&self)->usize{
        self.disponibilità
    }

    /** 
     * Funzione utile a creare in modo randomico un Nome e una disponibilità economica da assegnare 
     * al Partecipante nel momento della sua creazione. 
    */
    fn genera_nome_disponibilita()->(&'static str,usize){
        
        let arr_nomi = vec!["Luca","Noemi","Davide","Giorgia","Francesco","Federico","Giovanna"];
        let n = rand::thread_rng().gen_range(0..arr_nomi.len());
        
        let arr_valori = vec![1000,2000,3000,4000,5000,6000];
        let m = rand::thread_rng().gen_range(0..arr_valori.len());
        
        (arr_nomi[n],arr_valori[m])                    
    }

    /*
     * Metodo utile per avviare il Thread che determinarà il comportamento del participante all'asta.
     * Per ogni partecipante ci sarà un Sender con cui mandare messaggi verso il Banditore e un Receiver con cui ricevere messaggi dal Banditore.
     * Inizialmente il partecipante invia un messaggio in cui annuncia che parteciperà all'asta. 
     * Dopodiché, attende il primo messaggio di informazioni riguardante l'asta. Poi sceglie randomicamente se effettuare una offerta o uscire dall'asta.
     * Se appena prima di uscire dall'asta un altro partecipante A invia un'offerta prima che il partecipante B esca dall'asta allora l'informazione
     * riguardante questa offerta arriverà anche al partecipante B ma non avrà la possibilità di effettuare una offerta (gestito grazie ad asta di bool).
     * L'aumento dell'offerta viene scelto anch'esso randomicamente.
     * Una volta che è rimasto un solo partecipante attivo vengono stampati i messaggi di fine asta che dipendono da come si è evoluta.
     * Fine comportamento Partecipante.
    */

    /** 
     * Metodo utile ad avviare il comportamento di un partecipante all'interno dell'asta.
    */
    pub fn avvia_thread(&self, tx:Sender<Messaggio>,rx:Receiver<Messaggio>,risultato:Arc<Mutex<RisultatoAsta>>) -> thread::JoinHandle<()>{

        let id_partecipante = self.get_id();
        let nome_partecipante = self.get_nome().clone();
        let mut asta = true;

        let thread_partecipante = thread::spawn(move || {

            let _ = tx.send(Messaggio::Partecipo { id: id_partecipante, nome: nome_partecipante.clone() });

            while let Ok(msg) = rx.recv() {
                
                match msg{

                    Messaggio::Informazioni { descrizione_prodotto, prezzo_attuale } => {

                        {
                            let r = risultato.lock().unwrap();
                            // Verifico se è la prima informazione che arriva dal banditore riguardante l'asta.
                            if r.get_prezzo_venduto() == prezzo_attuale{
                                println!("[Partecipante Nome:{}, Id:{}] Ho ricevuto le informazioni iniziali dell'asta [Prezzo_iniziale:{}, Descrizione:{}]",nome_partecipante.clone(),id_partecipante,prezzo_attuale,descrizione_prodotto); 
                            }
                        }

                        if asta {
                            match Moneta::lancio_moneta() {
                                Moneta::Testa => {
                                    // Faccio un'offerta
                                    let aumento_offerta = rand::thread_rng().gen_range(20..=25);
                                    let nuova_offerta = aumento_offerta as f32 + prezzo_attuale;
                                    let _ = tx.send(Messaggio::Offerta { nome_partecipante: nome_partecipante.clone(), id_partecipante: id_partecipante, prezzo: nuova_offerta });
                                },
                                Moneta::Croce => {
                                    // Esco dall'asta
                                    let _ = tx.send(Messaggio::Esci { nome_partecipante: nome_partecipante.clone(), id_partecipante: id_partecipante });
                                    asta = false;
                                },  
                            }
                        }
                        
                    },
                    Messaggio::FineAsta => {
                        // Ricevuto il messaggio di fine asta 
                        {   
                            let r = risultato.lock().unwrap();

                            // Se il prezzo a cui è stato venduto il prodotto è maggiore uguale al prezzo di riserva allora stampo messaggio positivo.
                            if r.get_prezzo_venduto() >= r.get_prezzo_riserva(){
                                println!("[Partecipante Nome:{}, Id:{}]  Asta terminata. L'asta è stata vinta dal partecipante di nome {} con id {} al seguente prezzo {}.",
                                nome_partecipante,id_partecipante,r.get_nome_partecipante_vincitore().as_ref().unwrap(),r.get_id_partecipante().as_ref().unwrap(),r.get_prezzo_venduto());
                            }
                            else {
                                // Altrimenti stampo un messaggio di fine asta in cui non è stato raggiunto il prezzo di riserva e verifico se è stata fatta un'offerta o meno.
                                if r.get_id_partecipante().is_none(){
                                    println!("[Partecipante Nome:{}, Id:{}]  Asta terminata. L'asta non ha raggiunto il prezzo di riserva poiché non c'è stata nessuna offerta.",
                                    nome_partecipante,id_partecipante);   
                                }
                                else {
                                    println!("[Partecipante Nome:{}, Id:{}]  Asta terminata. L'asta non ha raggiunto il prezzo di riserva. [Prezzo riserva:{}, Prezzo raggiunto:{} dal partecipante con id:{}]",
                                    nome_partecipante,id_partecipante,r.get_prezzo_riserva(),r.get_prezzo_venduto(),r.get_id_partecipante().as_ref().unwrap());   
                                }
                                
                            } 

                        }
                        break;                        

                    },
                    _ => (),

                }

            }

        });
        thread_partecipante
    }
}