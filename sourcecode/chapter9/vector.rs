use std::mem::size_of; 
fn main() { 
  let class_student_ids:Vec<i64> = vec![11,12,13]; 	// 此处分配堆内存，被类型推定为vec<i32>
  println!("value class_student_ids: {}", std::mem::size_of_val(&class_student_ids)); 
  // println!("value class_student_ids[0]: {}", std::mem::size_of_val(&class_student_ids[0])); 
} 