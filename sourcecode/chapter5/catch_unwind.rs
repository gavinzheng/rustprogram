// catch_unwind.rs

use std::panic; 

fn main() { 
    panic::catch_unwind(|| { 
        panic!("Panicking!"); 
    }).ok();

    println!("Running after panic."); 
}