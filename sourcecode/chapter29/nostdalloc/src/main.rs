#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, vec};
use libc_alloc::LibcAlloc;
use libc_print::std_name::println;

#[global_allocator]
static GLOBAL_ALLOC: LibcAlloc = LibcAlloc;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe {
        libc::abort();
    }
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    println!("hello world!");
    let test1 = vec![1, 2, 3];
    for v in test1 {
        println!("{v}")
    }
    let test2 = Box::new(10);
    println!("{test2}");
    0
}