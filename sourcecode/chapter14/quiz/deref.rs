use std::ops::Deref;
struct MyBox<T> (T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}
impl<T> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target { 
    &self.0
  }
}

fn main(){
  let x:i32 =5; 
  let y= MyBox::new(x);

  let m = MyBox(String::from("gavin"));
  hello(&m);
}

fn hello(name : &str) {
  println!("Hello {}!",name);
}