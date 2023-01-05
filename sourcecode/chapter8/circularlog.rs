use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Node {
    value: i32,
    next: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}
fn main() {
  let a = Node::new(5);
  println!("a initial rc count = {}", Rc::strong_count(&a));
  let b = Node::new(10);
  println!("b initial rc count = {}", Rc::strong_count(&b));

  a.borrow_mut().next = Some(b.clone());
  println!("a next item = {:?}", a);
  println!("b rc count after changing a = {}", Rc::strong_count(&b));
  println!("a rc count after changing a = {}", Rc::strong_count(&a));

  b.borrow_mut().next = Some(a.clone());
  // println!("b next item = {:?}", b);
  println!("b rc count after changing b = {}", Rc::strong_count(&b));
  println!("a rc count after changing b = {}", Rc::strong_count(&a));
}