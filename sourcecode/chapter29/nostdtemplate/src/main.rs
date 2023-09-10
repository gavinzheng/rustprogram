#![no_std]
#![no_main]

// libc crate可以使用C语言的函数
extern crate libc; 
// A list of C functions that are being imported 
extern { 
    pub fn printf(format: *const u8, ...) -> i32; 
} 

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe{
        printf(b"Hello, World!\n" as *const u8); 
    }
    
    loop {}
    // 使用printf打印"Hello, World" 



}
