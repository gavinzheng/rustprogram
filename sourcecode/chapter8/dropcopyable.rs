use std::mem::drop; 
fn main() { 
  let x = 100_i32; 
  println!("drop前 {}", x); 
  drop(x); 
  println!("drop后 {}", x); 
} 