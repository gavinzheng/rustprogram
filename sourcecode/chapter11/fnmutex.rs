fn main() { 
  let mut hello = String::from("Hey!"); 
  let mut fn_mut_closure = || {
    hello.push_str("北科信链"); 	// 闭包修改了a变量：向a变量中增加“北科信链"
  }; 
  fn_mut_closure(); 
  println!("Main greets: {}", hello ); 
  } 