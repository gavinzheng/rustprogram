extern { 
  pub fn foo(arg: extern fn() -> *const c_char); 
  } 
  extern "C" fn danger() -> *const c_char { 
    let cstring = CString::new("I'm a danger string").unwrap(); 
    cstring.as_ptr() 
  } // 由于CString是owned类型，在这里cstring被rust free掉了。USE AFTER FREE! too young! 
  fn main() { 
    unsafe { 
      foo(danger); // boom !! 
    } 
} 