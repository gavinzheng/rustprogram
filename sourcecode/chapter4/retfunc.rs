fn main() { 
  let a = [11,22,33,44,55,66,77]; 
  let mut v = Vec::<i32>::new(); 
  for i in &a { 
  v.push(get_func(*i)(*i)); 
  } 
  println!("{:?}", v); 
  } 
  type timesType = fn(i32) -> i32;//函数类型 
  
  fn get_func(n: i32) -> fn(i32) -> i32 { 
  fn double(n: i32) -> i32 { 		// 在函数中定义函数
  n * 2
  } 
  fn triple(n: i32) -> i32 { 		// 在函数中定义函数
  n * 3
  } 
  if n % 2 == 0 { 
  double 
  } else { 
  triple
  } 
  }