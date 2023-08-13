macro_rules! swap{ 
  ($a:expr, $b:expr) => { 
  temp = $b; $b = $a; $a = temp; 	// 使用宏外部的变量temp
  }; 
  } 
  fn main() { 
  let x = 10; 
  let y = 20; 
  let temp = 30; 		// 虽然在宏外部定义了temp，但是宏并不能使用
  swap!(x, y); 
  } 