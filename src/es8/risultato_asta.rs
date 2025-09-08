/** 
 * Struttura che rappresenta il Risultato dell'Asta.
 * Qui si indicano il Partecipante che ha vinto l'asta tramite il suo nome e il suo identificato,
 * il prodotto vinto all'asta tramite il suo nome e il suo identificativo e infine il prezzo a cui è stato venduto.
*/
#[derive(Clone)]
pub struct RisultatoAsta{
    nome_partecipante_vincente: Option<String>,
    nome_prodotto: String,
    id_partecipante: Option<usize>,
    id_prodotto: usize,
    prezzo_venduto: f32,
    prezzo_riserva: f32,
}

impl RisultatoAsta{

    /** 
     * Costruttore che utile a creare il Risultato dell'asta. 
    */
    pub fn new(nome_part:Option<String>,nome_prod:String,id_part:Option<usize>,id_prod:usize,prezzo:f32,prezzo_riserva:f32)->Self{
        RisultatoAsta{nome_partecipante_vincente:nome_part,nome_prodotto:nome_prod,id_partecipante:id_part,id_prodotto:id_prod,prezzo_venduto:prezzo,prezzo_riserva:prezzo_riserva}
    }

    pub fn get_nome_partecipante_vincitore(&self)-> Option<String>{
        //&self.nome_partecipante_vincente.as_ref().unwrap()
        if self.nome_partecipante_vincente.is_none(){
            None
        }
        else{
            self.nome_partecipante_vincente.clone()
        }
    }

    pub fn get_nome_prodotto(&self)-> &String{
        &self.nome_prodotto
    }

    pub fn get_id_partecipante(&self)-> Option<usize>{
        if self.id_partecipante.is_none(){
            None
        }
        else{
            self.id_partecipante
        }
    }

    pub fn get_id_prodotto(&self)-> usize{
        self.id_prodotto
    }

    pub fn get_prezzo_venduto(&self)-> f32{
        self.prezzo_venduto
    }

    pub fn get_prezzo_riserva(&self)-> f32{
        self.prezzo_riserva
    }

    pub fn set_prezzo_venduto(&mut self, prezzo:f32){
        self.prezzo_venduto = prezzo;
    }

    pub fn set_id_partecipante(&mut self,id:usize){
        self.id_partecipante = Some(id);   
    }

    pub fn set_nome_partecipante(&mut self, nome:String){
        self.nome_partecipante_vincente = Some(nome);
    }

}