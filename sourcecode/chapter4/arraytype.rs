fn show(arr: [u8;3]) { 
  for i in &arr { 
        print!("{} ", i); 
  } 
  }
  fn main() { 
  let a: [u8; 3] = [1, 2, 3]; 
  show(a); 
  let b: [u8; 4] = [1, 2, 3, 4]; 
  show(b); 
  } 