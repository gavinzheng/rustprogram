struct Point { 		
  x: i32, 	
  y: i32, 
  }
  fn main() { 
  let mut point = Point { x: 0, y: 0 }; 	// 设定整个结构变量的可变性
  point.x = 100; 
  println!("点坐标是 ({}, {})", point.x, point.y); 
  } 