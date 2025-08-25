use crate::es6::node::{LinkedList, Node};
/** 
 * Questa è una struct che descrive una lista formata da:
 * - campo head che rappresenta la testa della lista ed è di tipo LinkedList<T>,
 * - campo size che rappresenta la lunghezza della lista.
*/
pub struct List<T> {
    head: LinkedList<T>,
    size: usize,
}

impl <T> List<T> {
    
    /** 
     * Costruttore utile per creare una Lista.
    */
    pub fn new() -> List<T>{
        List{head: None ,size:0}
    }
    
    /** 
     * Metodo utile ad aggiungere un nuovo elemento nella lista in base alla posizione indicata.
     * Se la posizione è maggiore della lunghezza della lista inserisce il nodo in fondo alla lista.
    */
    pub fn add_el(& mut self,pos:usize,data:T){

        /* 
         * Se la lista è vuota o si vuole inserire il nodo nella prima posizione
         * Creo un nuovo nodo prendendo con la take() il nodo successivo se esiste,
         * e assegno alla testa della lista il nuovo nodo creato.
         * Infine incremento la dimensione della lista.
         */
        if self.is_empty() || pos == 0{
            let new_nodo = Some(Box::new(Node::new(data, self.head.take())));
            self.head = new_nodo;
            self.size += 1;
        }
        else {

            /* 
             * Se voglio fare un inserimento in una posizione diversa dall'inizio della lista,
             * devo prima scorrere fino al nodo "N" precedente alla posizione di inserimento.
             * node_temp serve proprio per iterare sulla lista fino a quel nodo "N".
             * Una volta trovato, salvo temporaneamente il campo next di "N" in node_old e quindi,
             * creo il nuovo nodo node_new che punta al campo next di "N" salvato in precedenza.
             * Infine, aggiorno next del nodo "N" in modo tale che punti al nuovo nodo appena creato.
            */

            let mut i = 0;
            let mut node_temp = self.head.as_deref_mut();
            self.size += 1;
            
            while let Some(node) = node_temp {
                
                if pos-1 == i{
                    let node_old = node.get_mut_next().take();
                    let node_new = Some(Box::new(Node::new(data, node_old)));
                    node.set_next(node_new);
                    return;
                }

                /* 
                 * Se il nodo attuale nel campo next è None vuol dire che siamo arrivati alla fine della lista,
                 * quindi è inutile scorrerla ancora perciò, aggiungo come da indicazioni, il nodo alla fine della lista
                 * anche se la posizione in cui devo aggiungere questo nodo non è quella corretta. 
                */
                if node.get_next().is_none(){
                    let node_old = node.get_mut_next().take();
                    let node_new = Some(Box::new(Node::new(data, node_old)));
                    node.set_next(node_new);
                    return;
                }

                node_temp = node.get_mut_next().as_deref_mut(); // per prendere il nodo successivo e scorrere la lista
                i += 1;

            }
            
        }
    }

    /** 
     * Metodo utile a rimuovere un elemento nella lista in base alla posizione indicata.
     * Se la posizione è maggiore della lunghezza della lista oppure se la lista è vuota 
     * restituisce None altrimenti ritorna l'elemento eliminato.
    */
    pub fn rm_el(& mut self,pos:usize) -> Option<Node<T>>{

        /* 
         * Se la lista è vuota o la posizione richiesta è maggiore o uguale 
         * alla dimensione della lista allora ritorno None e non rimuovo nessun elemento.
        */
        if self.is_empty() || pos >= self.size {
            return None
        }

        /* 
         * Se la posizione da rimuovere è la 0 bisogna aggiornare la testa della lista.
         * Prendo il riferimento alla testa della lista e se è diversa da None allora
         * salvo il nodo da ritornare nella variabile risultato (attraverso unwrap) 
         * e aggiorno la testa della lista andando a prendere dal risultato il campo nodo
         * presente nel campo next che diventerà appunto la nuova testa della lista.
         * Infine, aggiorno la dimensione della lista e ritorno l'elemento rimosso.
        */
        if pos == 0{
            let nodo_rimosso = self.head.take();
            if nodo_rimosso.is_some(){
                let mut risultato = nodo_rimosso.unwrap();
                self.head = risultato.get_mut_next().take();
                self.size -= 1;
                return Some(*risultato)
            }
        }

         /* 
          * Se voglio rimuovere un elemento in una posizione diversa dall'inizio della lista,
          * devo prima scorrere fino al nodo "N" precedente a quello da eliminare.
          * node_temp serve proprio per iterare sulla lista fino a quel nodo "N".
          * Una volta trovato, salvo temporaneamente il campo next di "N" in nodo_rimosso poiché 
          * è appunto il nodo da rimuovere.
          * Se tale nodo è diverso da None allora salvo il nodo da ritornare nella variabile risultato 
          * (attraverso unwrap) e aggiorno il campo next del nodo che usato per scorrere la lista,
          * facendo in modo che il suo campo next sia pari al campo next del nodo da eliminare.
          * Infine, aggiorno la dimensione della lista e ritorno l'elemento rimosso.
        */
        let mut node_temp = self.head.as_deref_mut();
        let mut i = 0;

        while let Some(nodo) = node_temp {
            
            if pos-1 == i{
                let nodo_rimosso= nodo.get_mut_next().take();
                if nodo_rimosso.is_some(){
                    let mut risultato = nodo_rimosso.unwrap();
                    nodo.set_next(risultato.get_mut_next().take());
                    self.size -= 1;
                    return Some(*risultato)
                }
            }

            node_temp = nodo.get_mut_next().as_deref_mut();
            i += 1;
        }

        None
    }

    /** 
     * Metodo utile ad  un nuovo elemento nella lista.
     * Se la posizione è maggiore della lunghezza della lista oppure se la lista è vuota 
     * restituisce None altrimenti ritorna l'elemento eliminato.
    */
    pub fn get_el(& mut self,pos:usize) -> Option<&T>{
        
        /* 
         * Se la lista è vuota o la posizione richiesta è maggiore o uguale 
         * alla dimensione della lista allora ritorno None.
        */
        if self.is_empty() || pos >= self.list_len() {
            return None
        }

        let mut i = 0;
        let mut node_temp = self.head.as_deref();
        
        /* 
         * Altrimenti scorro la lista fino alla posizione 
         * indicata e ritorno l'elemento.
        */
        while let Some(nodo) = node_temp {
            
            if pos == i{
                return Some(&nodo.get_data())
            }

            node_temp = nodo.get_next().as_deref();
            i += 1;
        }
        None

    }

    /** 
     * Metodo che ritorna la lunghezza della lista.
    */
    pub fn list_len(&self) -> usize{
        self.size
    }

    /**
     * Metodo che indica se la lista è vuota
     */
    pub fn is_empty(&self) -> bool{
        self.head.is_none()
    }

}