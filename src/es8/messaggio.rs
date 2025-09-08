/** 
 * Enum utile a rappresentare i tipi di messaggio 
 * che si possono scambiare i Thread che rappresentano i Partecipanti e il Banditore.
*/
pub enum Messaggio {
    Partecipo {id:usize,nome:String},
    Offerta {nome_partecipante:String,id_partecipante: usize, prezzo: f32},
    Esci {nome_partecipante:String, id_partecipante:usize},
    Informazioni {descrizione_prodotto: String, prezzo_attuale: f32},
    FineAsta,
}