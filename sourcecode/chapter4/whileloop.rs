fn main() { 
  let mut n = 1; 		// 计数器变量
  
  // 当 `n` 小于 20 时进入循环操作 
  while n < 20 { 
  if n % 10 == 0 { 
  break; 		// 如果n是10的整数倍，则退出while循环
  } 
  // 计数器值加1 
  n += 1; 
  } 
  }