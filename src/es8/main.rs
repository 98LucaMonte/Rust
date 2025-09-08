use esercizi_esame_rust::es8::{asta::Asta, banditore::Banditore, partecipante::Partecipante, prodotto::Prodotto};
use rand::Rng;

fn main(){
    println!("Benvenuto all'interno dell'asta inglese!!");

    let numero_partecipanti = rand::thread_rng().gen_range(3..=5);
    println!("\nI partecipanti in questa asta saranno: {}.\n",numero_partecipanti);

    let mut array_partecipanti = vec![];
    for id in 1..=numero_partecipanti{
        let partecipante = Partecipante::new(id);
        array_partecipanti.push(partecipante);
    }

    let banditore = Banditore::new("Luigi");
    let prodotto = Prodotto::new("Portagioie re luigi xvi".to_string(), 
                "Portagioie appartenuto a re luigi xvi".to_string(), 130.0, 
                170.0, 100);

    let mut asta = Asta::new(banditore,array_partecipanti,prodotto);
    asta.avvia_asta();
    println!("\n\nÈ terminata l'asta!!!");
}