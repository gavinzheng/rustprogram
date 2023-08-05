fn main() {
  // 指定类型 
  let v1: Vec<u8> = Vec::new(); 
  // 或者调用方法 
  let mut v2 = Vec::new(); 
  v2.push(2); // v2 类型推定为Vec<i32> 
  // 使用：： 
  let v3 = Vec::<u8>::new(); // 指定T为u8 
  }