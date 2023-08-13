fn main() { 
  let v = (0..5).map(|x| x * x); 		// 此处并没有表达式求值
   
  for x in v { 
  println!("{}", x); 				// 此处才开始执行 x*x 
  } 
  } 