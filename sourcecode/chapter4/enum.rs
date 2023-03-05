use std::collections::HashMap; 

enum Json { 							// 枚举类型为Json
  Null, 								// 不带数据的分支
  Boolean(bool), 						// 类似元组结构体的带布尔数据的分支
  Number(f64), 						// 类似元组结构体的带浮点数数据的分支
  String(String), 					// 类似元组结构体的带字符串数据的分支
  Array(Vec<Json>), 					// 类似元组结构体的带Json类型的矢量数据的分支
  Object(Box<HashMap<String, Json>>), 	// 类似元组结构体的带字符串到Json映射指针数据的分支
}
// const fn size_of_enum_data<T>() -> usize {
//   let a = std::mem::align_of::<T>();
//   println!("a={}", a);
//   let i = std::mem::size_of::<i32>();
//   println!("i={}", i);
//   std::mem::size_of::<T>() - if a > i { a }  else { i }
// }
fn main() { 
  // 使用了泛型函数的调用语法,请参考第21章泛型 
  println!("Size of Json: {}", std::mem::size_of::<Json>()); 
  // println!("Size of i32: {}", std::mem::size_of::<Json::Null>()); 
  println!("Size of bool: {}", std::mem::size_of::<bool>()); 
  println!("Size of f64: {}", std::mem::size_of::<f64>()); 
  println!("Size of String: {}", std::mem::size_of::<String>()); 
  println!("Size of Vector: {}", std::mem::size_of::<Vec<Json>>()); 
  println!("Size of Box: {}", std::mem::size_of::<Box<HashMap<String, Json>>>()); 
  // println!("Size of Json without Tag: {}", size_of_enum_data::<Json>());
  println!("Size of Json Tag + alignment: {}", std::mem::align_of::<Json>());
} 