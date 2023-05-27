/*#![feature(alloc_system, global_allocator, allocator_api)]

extern crate alloc_system;

use alloc_system::System;

#[global_allocator]
static A: System = System;
*/
/*#![feature(alloc_system, global_allocator, allocator_api)]

extern crate alloc_system;

#[global_allocator]
static A: alloc_system::System = alloc_system::System;
*/
use std::alloc::System;

#[global_allocator]
static A: System = System;

fn main() {
    println!("Hello, world!");
}
