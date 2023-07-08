use std::mem::size_of; 
use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
fn main(){
  // assert_eq!(size_of::<Ordering>(), 1); 
  // assert_eq!(size_of::<HttpStatus>(), 2); 

  std::mem::size_of::<*mut u8>(); // 8字节原始指针
  std::mem::size_of::<*mut [u8]>(); // 16字节胖指针= 8字节指针 + 8字节长度
  std::mem::size_of::<*mut [u8;4]>(); // 8字节原始指针
  std::mem::size_of::<*mut str>(); // 16 字节胖指针 = 8字节指针 + 8字节长度
  std::mem::size_of::<*mut dyn Drop>(); // 16 字节胖指针 = 8字节指针 + 8字节虚拟表

  println!("type u8: {}", std::mem::size_of::<u8>()); 
  println!("type f64: {}", std::mem::size_of::<f64>()); 
  println!("value 4u8: {}", std::mem::size_of_val(&4u8)); 
  println!("value 4: {}", std::mem::size_of_val(&4)); 
  println!("value 'a': {}", std::mem::size_of_val(&'a')); 
  println!("value \"Hello World\" as a static str slice: {}", 
  std::mem::size_of_val("Hello World")); 
  println!("value \"Hello World\" as a String: {}",  
  std::mem::size_of_val("Hello World").to_string()); 
  println!("Cell(4)): {}", std::mem::size_of_val(&Cell::new(84))); 
  println!("RefCell(4)): {}", std::mem::size_of_val(&RefCell::new(4))); 
  println!("Rc(4): {}", std::mem::size_of_val(&Rc::new(4))); 
  println!("Rc<RefCell(8)>): {}", 
  std::mem::size_of_val(&Rc::new(RefCell::new(4))));

  print!("{} {} {} {} {} {} {} {} {} {} {} {}", 
  size_of::<i8>(), size_of::<u8>(), size_of::<i16>(), size_of::<u16>(), size_of::<i32>(), 
  size_of::<u32>(), size_of::<i64>(), size_of::<u64>(), size_of::<f32>(), size_of::<f64>(), 
  size_of::<bool>(), size_of::<char>());
}