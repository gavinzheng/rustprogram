use std::fmt::Debug; 
#[derive(Clone, Debug)] 
struct S; 
#[derive(Debug)] 
struct R<T: Debug> { 
  x: *const T 
} 
fn main() { 
  let mut r = R { x: std::ptr::null() }; 
  { 
    let local = S{}; 
    r.x = &local; 
  } 
  // r.x now is dangling pointer 
} 