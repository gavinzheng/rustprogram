use std::process; 
fn main() { 
  println!("Going to abort process"); 
  process::abort(); 
  println!("Process aborted"); 	// Statement will not be executed
}