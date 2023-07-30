struct S { 
  field1: char, 
  field2: bool, 
  } 
  fn soemFunc( S{field1: arg1, field2: arg2} : S) { 
  println!("{} {}", arg1, arg2); 
  } 
  fn main() 
  { 
  let x = S { 
  field1: 'Z', 
  field2: true, 
  }; 
  soemFunc(x); 
  } 