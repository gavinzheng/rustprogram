use std::process; 
fn main() { 
println!("Going to abort process"); 
process::abort(); 
println!("Process aborted"); 	// 这条语句不会被执行
}