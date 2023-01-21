#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    let mut generator = || {
        for i in 0..10 {
            yield i;
        }
        return "Finished!";
    };

    loop {
        match Pin::new(&mut generator).resume(()) {
            GeneratorState::Yielded(num) => println!("Yielded {}", num),
            GeneratorState::Complete(text) => {
                println!("{}", text);
                break;
            }
        }
    }
}
