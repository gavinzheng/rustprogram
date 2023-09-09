fn main() {
  let student = "milly".to_owned();  // 此处分配堆内存
  println!("{}", std::mem::size_of::<String>());
  println!("Capacity: {} len: {}", student.capacity(),student.len());
  // println!("{}", std::mem::size_of_val(student));
}  // student离开其作用域，则调用析构函数Drop 