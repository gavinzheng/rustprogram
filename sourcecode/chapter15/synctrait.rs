use std::thread;
use std::cell::Cell;
#[derive(Debug,Copy,Clone)]
struct Foo { 
  number: u8 
} 
fn main() {
  let v = vec![1, 2, 3];
  let foo1 = Cell::new(Foo{number:1}); 

  let handle = thread::spawn(move || {
      let ref_foo_1 = &foo1; 
      foo1.set(Foo{number:100}) ;
      println!("Here's a vector: {:?} {:?}", v,ref_foo_1.get().number);
  });
   

  handle.join().unwrap();
}