use std::process; 
fn main() { 
  println!("Going to exit process with error code 64"); 
  process::exit(64); 			      // 可以返回一个错误代码给OS
  println!("Process exited"); 	// Statement will not be executed

}