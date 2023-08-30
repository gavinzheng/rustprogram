#![feature(box_syntax)] 
struct S{ 
val: i32 
} 
fn main() { 
let p : Box<S> = box S{val: 100}; 
println!("{}", p.val); 
} 
// #![feature(rustc_attrs)]
// #![rustc_box] 