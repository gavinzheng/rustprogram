enum UnionSizeIsNot8Bytes  {
  U16(u16),
  U8(i32),
  // Float(f64),
  //String(String), 
 }

 fn size_of_enum_data<T>() -> usize {
  let a = std::mem::align_of::<T>();
  println!("a={}", a);
  let i = std::mem::size_of::<i32>();
  println!("i={}", i);
  println!("std::mem::size_of::<T>()={}", std::mem::size_of::<T>());
  std::mem::size_of::<T>() - if a > i { a }  else { i }

}
fn main() {
  println!("Size of UnionSizeIsNot8Bytes : {}", std::mem::size_of::<UnionSizeIsNot8Bytes>());
  println!("Size of u16: {}", std::mem::size_of::<u16>());
  println!("Size of u8: {}", std::mem::size_of::<i32>());
  println!("Size of UnionSizeIsNot8Bytes without Tag: {}", size_of_enum_data::<UnionSizeIsNot8Bytes>());
  // println!("Size of String: {}", std::mem::size_of::<String>());
 }