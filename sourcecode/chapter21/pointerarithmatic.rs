fn main() { 
  let list= [100u32, 200, 300, 400, 500]; 
  let ptr = &list[2] as *const u32; 
  println!("{}", unsafe { *ptr }); 
  println!("{}", unsafe { *ptr.offset(-2) }); 
  println!("{}", unsafe { *ptr.offset(2) }); 
} 