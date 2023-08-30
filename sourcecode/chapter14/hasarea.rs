trait HasArea { 
  fn area(&self) -> f64; 
} 
impl HasArea for i32 { 
  fn area(&self) -> f64 { 
    (*self * *self) as f64 
  } 
} 

fn main() {
  println!("area is {}",  5.area());
} 