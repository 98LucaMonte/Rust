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

        if self.is_empty() || pos == 0{
            let new_nodo = Some(Box::new(Node::new(data, self.head.take())));
            self.head = new_nodo;
            self.size += 1;
        }
        else {
            let mut i = 0;
            let mut node_temp = self.head.as_deref_mut();
            self.size += 1;
            
            while let Some(node) = node_temp {
                
                if pos-1 == i{
                    let node_old = node.next.take();
                    let node_new = Some(Box::new(Node::new(data, node_old)));
                    node.next = node_new;
                    return;
                }

                if node.next.is_none(){
                    let node_old = node.next.take();
                    let node_new = Some(Box::new(Node::new(data, node_old)));
                    node.next = node_new;
                    return;
                }

                node_temp = node.next.as_deref_mut();
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

        if self.is_empty() || pos >= self.size {
            return None
        }

        if pos == 0{
            let nodo_rimosso = self.head.take();
            if nodo_rimosso.is_some(){
                let mut risultato = nodo_rimosso.unwrap();
                self.head = risultato.next.take();
                self.size -= 1;
                return Some(*risultato)
            }
        }

        let mut node_temp = self.head.as_deref_mut();
        let mut i = 0;

        while let Some(nodo) = node_temp {
            
            if pos-1 == i{
                let nodo_rimosso= nodo.next.take();
                if nodo_rimosso.is_some(){
                    let mut risultato = nodo_rimosso.unwrap();
                    nodo.next = risultato.next.take();
                    self.size -= 1;
                    return Some(*risultato)
                }
            }

            node_temp = nodo.next.as_deref_mut();
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
        
        if self.is_empty() || pos >= self.size {
            return None
        }

        let mut i = 0;
        let mut node_temp = self.head.as_deref();
        
        while let Some(nodo) = node_temp {
            
            if pos == i{
                return Some(&nodo.get_data())
            }

            node_temp = nodo.next.as_deref();
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