fn main() { 
  let x = loop { 						// loop语句的返回值绑定到x变量
  break 100; 					//  loop语句的返回值为100
  }; 
  println!("{}", x); 
  
  let n = 12; 
  let is_even = |x: u64| -> bool { x % 2 == 0 }; 
  let description = if is_even(n) {		// if语句的返回值绑定到description变量 
  "even number" 
  } else { 
  "odd number" 
  }; 
  println!("{} is {}", n, description); 
  
  let mut optional = Some(0); 
  // 分析：当 `let` 将 `optional` 解构成 `Some(i)` 时，就 
  // 执行语句块(`{}`)。否则中断退出(`break`)。 
  while let Some(i) = optional { 	// 当optional为None时退出
  if i > 9 { 
  println!("Greater than 9, quit!"); 
  optional = None; 
  } else { 
  println!("`i` is `{:?}`. Try again.", i); 
  optional = Some(i + 1); 
  } 
  } 
  
  }