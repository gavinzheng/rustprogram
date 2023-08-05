struct ZeroStruct{} 

fn main() { 
  let mut v = Vec::<ZeroStruct>::new(); 
  println!("capacity:{} length:{}", v.capacity(), v.len()); 
  v.push(ZeroStruct{}); 
  v.push(ZeroStruct{}); 
  println!("capacity:{} length:{}", v.capacity(), v.len()); 
  
  // p 永远指向 align_of::<ZeroStruct>(),不需要调用 allocator 
  let p = v.as_ptr(); 
  println!("ptr:{:p}", p); 
  let size1 = std::mem::size_of::<Vec<i32>>(); 
  let size2 = std::mem::size_of::<Option<Vec<i32>>>(); 
  println!("size of Vec:{} size of option vec:{}", size1, size2); 
} 