/** 
 * Questa è una struct che descrive un nodo della lista ed è formata da:
 * - campo data T,
 * - campo next che punta ad un tipo LinkedList<T>.
*/
pub struct Node<T> {
    data: T,
    next: LinkedList<T>,
}

/**
 * Il LinkedList<T> è un tipo Option che contiene al suo interno un Box di Node<T>
 */
pub type LinkedList<T> = Option<Box<Node<T>>>;

impl <T> Node<T> {

    /** 
     * Costruttore utile per creare un Node.
    */
    pub fn new(data:T,next: LinkedList<T>)-> Node<T>{
        Node {  data: data,next: next }
    }

    /** 
     * Metodo utile per accedere al dato del nodo.
    */
    pub fn get_data(& self)-> &T{
        &self.data
    }

    /** 
     * Metodo utile per settare il dato del nodo.
    */
    pub fn set_data(&mut self,data: T) {
        self.data = data;
    }

    /** 
     * Metodo utile per prendere in prestito il campo next del nodo.
    */
    pub fn get_next(& self)-> &Option<Box<Node<T>>>{
        &self.next
    }

    /** 
     * Metodo utile per prendere in prestito mutabile il campo next del nodo.
    */
    pub fn get_mut_next(&mut self)-> &mut Option<Box<Node<T>>>{
        & mut self.next
    }

    /** 
     * Metodo utile per settare il campo next del nodo.
    */
    pub fn set_next(& mut self, nodo:Option<Box<Node<T>>> ){
        self.next = nodo;
    }

}