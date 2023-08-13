pub struct Rectangle {
  width: u32,
  length: u32
}

impl Rectangle {
  // fn area() => f64 {
  //   width * length
  // }
  // fn perimeter() => u32 {
  //   2 *(width + length) 
  // }
  pub fn whoami()  {
    println!("Current shape is Rectangle")
  }
}

pub mod square;