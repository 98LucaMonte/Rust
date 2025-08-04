use rand::Rng;
use std::io::*;
use esercizi_esame_rust::es3::campo::Campo; 
use esercizi_esame_rust::es3::giocatore::Giocatore;
use esercizi_esame_rust::es3::configurazione::Configurazione;
use esercizi_esame_rust::es3::livello::Livello;
  
fn main(){
    
    println!("Benvenuto nel gioco il labirinto della morte");

    let dimensione_campo = rand::thread_rng().gen_range(7..=10);
    println!("\nLa dimensione del campo {}x{}",dimensione_campo,dimensione_campo);

    println!("Seleziona il livello con cui vuoi giocare\nAumentando il livello di difficoltà ci saranno più celle velenose (che fanno più danni) e più celle curative\nScegli tra Facile, Medio o Difficile:");
    let mut scelta_livello = String::new();
    
    stdin().read_line(& mut scelta_livello).expect("Non è possibile leggere l'input dall'utente");
    let livello: Livello;
    match scelta_livello.trim() {
        "facile" | "Facile" => {
            livello = Livello::Facile;
            println!("Hai selezionato il livello {}",livello)
        },
        "medio" | "Medio" => {
            livello = Livello::Medio;
            println!("Hai selezionato il livello {}",livello)
        },
        "difficile" | "Difficile" => {
            livello = Livello::Difficile;
            println!("Hai selezionato il livello {}",livello)
        },
        _ =>{
            println!("Livello selezionato non disponibile, verrà quindi selezionato come livello di default\n il livello Medio");
            livello = Livello::Medio;
        }
    }

    let campo = Campo::new(dimensione_campo, &livello);
    let giocatore = Giocatore::new(&campo, &livello);
    let mut configurazione = Configurazione::new(campo, giocatore);

    configurazione.avvio_partita();

}