fn main() { 
  let student = "milly".to_owned(); 	// 此处分配堆内存
}  // student离开其作用域，则调用析构函数Drop 