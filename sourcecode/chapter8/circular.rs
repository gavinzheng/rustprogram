use std::rc::Rc;
use std::cell::RefCell;
// use crate::List::{Cons, Nil};

// #[derive(Debug)]
// enum List {
//  Cons(i32, RefCell<Rc<List>>),
//  Nil,
// }

#[derive(Debug)] 
struct LinkedList<T> { 
  head: Option<Rc<Node<T>>> 
} 

#[derive(Debug)] 
struct Node<T> { 
  next: Option<RefCell<Rc<Node<T>>>>, 	// 指向下一节点的指针（强引用计数）
  data: T 
} 

impl<T>  Node<T> {
  fn tail(&self) -> Option<&RefCell<Rc<Node<T>>>> {
    self.next.as_ref()
  }
  // fn deref_mut(&mut self) -> &mut RefCell<Rc<Node<T>>> {
	// 	match self.next{
  //     Some(ref item) => Some(&mut item)
  //     None => Some(RefCell::new(Rc)) 
  //   }
  // }
}
fn main() {
  let a = Rc::new(RefCell::new(Node::<i32>{data:5, next:None}));
  // a.next=Some(RefCell::new(a));
 
  println!("a initial rc count = {}", Rc::strong_count(&a));
  //println!("a next item = {:?}", a.tail());
  
  //let b = Rc::new(RefCell::new(Node::<i32>{data:10, next:Some(RefCell::new(Rc::clone(&a)))}));
  let b = Rc::new(RefCell::new(Node::<i32>{data:10, next:None}));
  println!("a rc count after b creation = {}", Rc::strong_count(&a));
  println!("b initial rc count = {}", Rc::strong_count(&b));
 // println!("b next item = {:?}", b.tail());
  
  a.borrow_mut().next = Some(b.clone());

  // if let Some(link) = a.tail() {
  //   *link.borrow_mut() = Rc::clone(&b);
  //   println!("001 a rc count after changing a = {} link={:?}", Rc::strong_count(&a),link); 
  // }
  // else{
  //   a.next = Some(RefCell::new(Rc::clone(&b)))
  // }

  // if let Some(link) = &a.next{
  //   *link.borrow_mut() = Rc::clone(&b);
  //   println!("a next item = {:?}", link);
  // }
  //a.next=Some(RefCell::new(Rc::clone(&b)));
  
  // println!("a next item = {:?}", a.tail());

  println!("b rc count after changing a = {}", Rc::strong_count(&b));
  println!("a rc count after changing a = {}", Rc::strong_count(&a));
  
  // Uncomment the next line to see that we have a cycle; 
  // it will overflow the stack.
  // println!("a next item = {:?}", a.tail());
}