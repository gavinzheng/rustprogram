use std::mem; 
fn main() { 
let v: &[u8] = unsafe { 
mem::transmute("北科信链") 
}; 
println!("{:?}", v); 
}