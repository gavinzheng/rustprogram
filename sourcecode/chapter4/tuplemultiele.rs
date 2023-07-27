fn main() { 
  let (p2,p3) = pow23(3); 
  println!("3的平方是{}.", p2); 
  println!("3的3次方是{}.", p3); 
  } 
  fn pow23(n: i32) -> (i32, i32) { 
  (n*n, n*n*n) 
  }