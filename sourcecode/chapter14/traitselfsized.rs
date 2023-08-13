trait Foo where Self: Sized { 
  fn foo(&self); 
  } 
  impl Foo for i32 { 
  fn foo(&self) { println!("{}", self); } 
  } 
  fn main() { 
  let x = 1_i32; 
  x.foo(); 
  let p = &x as &dyn Foo; 
  p.foo(); 
  } 