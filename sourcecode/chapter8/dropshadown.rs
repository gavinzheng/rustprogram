use std::ops::Drop; 

struct S(i32); 

impl Drop for S { 
  fn drop(&mut self) { 
    println!("析构 {}", self.0); 
  } 
} 
fn main() { 
  let x = S(100); 
  println!("构建第一个变量"); 
  let x = S(200); 
  println!("构建第二个变量"); 
} 