/** 
 * Questa è una struct che descrive un nodo della lista ed è formata da:
 * - campo data T,
 * - campo next che punta ad un tipo LinkedList<T>.
*/
pub struct Node<T> {
    data: T,
    pub next: LinkedList<T>,
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

}