fn mul_ten(x: u32) -> u32 { 
  x * 10
}
fn repeat(f: fn(u32) -> u32, arg: u32) -> u32 { 
  f(arg) + f(arg) 
} 
fn main() { 
  let my_func = mul_ten; 
  let result = my_func(5); 		// 函数指针保存为result变量
  let again = repeat(mul_ten, 2);	// 作为函数指针传入另外一个函数repeat
  
  println!("{:?} {:?}", result, again); 
} 