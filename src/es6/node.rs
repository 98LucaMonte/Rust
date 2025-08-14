pub struct Node<T> 
{
    pub data: T,
    pub next: LinkedList<T>,
}

pub type LinkedList<T> = Option<Box<Node<T>>>;

impl <T> Node<T> {

    pub fn new(data:T,next: LinkedList<T>)-> Node<T>{
        Node {  data: data,next: next }
    }

}