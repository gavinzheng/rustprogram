// thread_moves.rs

use std::thread;

// fn main() {
//     let my_str = String::from("Greeting to Milly");
//     let _ = thread::spawn(move || {
//         println!("In thread: {}", my_str);
//     });
//     println!("In main: {}", my_str);
// }

fn main() { 
    let my_greeting = String::from("Greeting to Milly!"); 
    let _ = thread::spawn(move || { 
        println!("In thread: {}", my_greeting ); 
    }); 
    // println!("In main: {}", my_greeting ); 
} 