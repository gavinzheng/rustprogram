extern "C" fn callback(a: c_int) { // 这个函数是传给c调用的 
  println!("hello {}!", a); 
  } 
  #[link(name = "yourlib")] 
  extern { 
  fn run_callback(data: i32, cb: extern fn(i32)); 
  } 
  fn main() { 
  unsafe { 
  run_callback(1 as i32, callback); // 打印 1 
  } 
  } 