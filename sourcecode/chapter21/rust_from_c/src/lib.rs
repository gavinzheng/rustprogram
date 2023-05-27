// rust_from_c/src/lib.rs

use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub enum Order {
    Greaterthan,
    Lessthan,
    Equal
}

#[no_mangle]
pub extern "C" fn compare_str(a: *const c_char, b: *const c_char) -> Order {
    let a = unsafe { CStr::from_ptr(a).to_bytes() };
    let b = unsafe { CStr::from_ptr(b).to_bytes() };
    if a > b {
        Order::Greaterthan
    } else if a < b {
        Order::Lessthan
    } else {
        Order::Equal
    }
}
