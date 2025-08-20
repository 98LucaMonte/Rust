/** 
 * Questa è una struct che descrive un nodo della lista ed è formata da:
 * - campo data T,
 * - campo next che punta ad un tipo LinkedList<T>.
*/
pub struct Node<T> 
{
    pub data: T,
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

    pub fn get_data(& self){
        todo!()
    }

    pub fn get_next(& self){
        todo!()
    }

    pub fn set_data(& mut self){
        todo!()
    }

    pub fn set_next(& mut self){
        todo!()
    }
}