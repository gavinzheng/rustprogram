use std::thread;
use std::rc::Rc;

fn main() {
  let v = vec![1, 2, 3];
  let pointer = Rc::new(1);

  let handle = thread::spawn(move || {
      println!("Here's a vector: {:?} {:?}", v,pointer);
  });

  handle.join().unwrap();
}