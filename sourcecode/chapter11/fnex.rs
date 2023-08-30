fn main() { 
  let hello = String::from("Hey, Milly!"); 
  let fn_closure = || { 
  println!("闭包搭讪: {}", hello ); 	// 闭包体内对hello 的读访问
  }; 
  fn_closure(); 
  println!("祝福: {}", hello ); 
  }