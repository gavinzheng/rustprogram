// #![feature(start, libc, lang_items)]
// #![no_std]
// #![no_main]
// // The libc crate allows importing functions from C.
// extern crate libc;
// // A list of C functions that are being imported
// extern {
//  pub fn printf(format: *const u8, ...) -> i32;
// }
// #[no_mangle]
// // The main function, with its input arguments ignored, and an exit status is returned
// pub extern fn main(_nargs: i32, _args: *const *const u8) -> i32 {
//  // Print "Hello, World" to stdout using printf
//  unsafe {
//  printf(b"Hello, World!\n" as *const u8);
//  }
//  // Exit with a return status of 0.
//  0
// }
// #[lang = "eh_personality"] extern fn eh_personality() {}
// #[lang = "panic_fmt"] extern fn panic_fmt() -> ! { panic!() }
#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate libc;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Since we are passing a C string the final null character is mandatory
    const HELLO: &'static str = "Hello, world!\n\0";
    unsafe {
        libc::printf(HELLO.as_ptr() as *const _);
    }
    0
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}