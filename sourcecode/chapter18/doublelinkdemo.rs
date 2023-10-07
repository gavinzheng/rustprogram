use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<LinkedListNode<T>>>
}

#[derive(Debug)]
struct LinkedListNode<T> {
    next: Option<Rc<LinkedListNode<T>>>,
    prev: RefCell<Option<Rc<LinkedListNode<T>>>>,
    data: T
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, data: T) -> Self {
        let new_node = Rc::new(LinkedListNode {
            data: data,
            next: self.head.clone(),
            prev: RefCell::new(None)
        });

        match self.head.clone() {
            Some(node) => {
                let mut prev = node.prev.borrow_mut();
                *prev = Some(new_node.clone()) ; // Some(Rc::downgrade(&new_node));
            },
            None => {
            }
        }
        // println!("head ={:?} newnode ={:?}", self.head, newnode);

        LinkedList {
            head: Some(new_node)
        }
    }
}

fn main() {
    // let numbers = LinkedList::new().append(10).append(20).append(30).append(40);
    let mut node1 = LinkedList::new().append(10).append(20);
    // let node2 = node1.append(20);
    node1= LinkedList::new() ; 
    // let node3 = node2.append(30);
    // let node4 = node3.append(40);

    // node1.head.unwrap().get().next = node3.head.next;

    //println!("numbers: {:?}", numbers);
}
