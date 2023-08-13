fn main() {
  let message = "name: milly\r\n  
    gender: male\r\n 
    \r\n  
    award: Ferrari!!!\r\n"; 
  
  let mut lines = message.lines(); 
  println!("Student:"); 
  for student in lines.by_ref()			// by_ref方法生成一个对student迭代子的可变引用
    .take_while(|x| !x.is_empty()) { 	// take_while工作在迭代子的可变引用上，并获得其所有权
    println!("{}" , student ); 
  } 								          // 此处对于lines的借用结束
  
  println!("\nAward:"); 
  for line in lines { 				// 所以，此处我们就可以访问lines
    println!("{}" , line); 
  } 
}