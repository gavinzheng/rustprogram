#![feature(asm)]

use std::arch::asm;

pub fn main() { unsafe { asm!("" : : : "hello", "milly") }; }