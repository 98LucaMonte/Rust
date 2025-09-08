
/** 
 * Struttura che rappresenta il Prodotto dell'asta.
 * È composta dal nome, dal suo identificativo, dalla descrizione, il prezzo di partenza e di riserva. 
*/
pub struct Prodotto {
    nome: String,
    descrizione: String,
    prezzo_partenza: f32,
    prezzo_riserva: f32,
    id: usize,
}

impl Prodotto {

    /** 
     * Costruttore utile a creare un nuovo Prodotto. 
    */
    pub fn new(nome:String,descrizione:String, prezzo_partenza:f32,prezzo_riserva:f32,id:usize)->Self{
        Prodotto{nome:nome,descrizione:descrizione,prezzo_partenza:prezzo_partenza,prezzo_riserva:prezzo_riserva,id:id}
    }

    /** 
     * Metodo utile per ritornare il nome del Prodotto.
    */
    pub fn get_nome(&self)->&String{
        &self.nome
    }

    /** 
     * Metodo utile per ritornare la descrizione del Prodotto.
    */
    pub fn get_descrizione(&self)->&String{
        &self.descrizione
    }
    
    /** 
     * Metodo utile per ritornare l'identificativo del Prodotto.
    */
    pub fn get_id(&self)->usize{
        self.id
    }

    /** 
     * Metodo utile per ritornare il prezzo di partenza del Prodotto.
    */
    pub fn get_prezzo_partenza(&self)->f32{
        self.prezzo_partenza
    }
    
    /** 
     * Metodo utile per ritornare il prezzo di riserva del Prodotto.
    */
    pub fn get_prezzo_riserva(&self)->f32{
        self.prezzo_riserva
    }
    
}