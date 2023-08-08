fn main() { 
  let hello = String::from("Hey!"); 
  let fn_closure = || { 
  println!("Closure accosts: {}", hello ); 	// 闭包体内对hello 的读访问
  }; 
  fn_closure(); 
  println!("Main greet: {}", hello ); 
  }