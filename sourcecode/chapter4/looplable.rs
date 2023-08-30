fn main() {
'outer: loop { 
  println!("进入外部循环"); 
  'inner: loop { 
  println!("进入内部循环"); 
  // break; 		// 这里的break语句会跳出内部循环 inner
  break 'outer; 	// 这里的break语句会跳出外部循环 outer
  } 
  println!("此处永远不可能被执行！！！"); 
  } 
  println!("退出外部循环!"); 
}