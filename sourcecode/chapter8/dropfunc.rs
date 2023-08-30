use std::mem::drop; 
fn main() { 
  let mut vec = vec![10, 20, 30]; 	// <--- v的生命周期开始 
  drop(vec); 					// ---> v的生命周期结束 
  v.push(100); 					// 错误的调用 
}  