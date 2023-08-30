fn main() {
  let mut s = "hello"; 
  s[0] = 'c'; 	// 错误： `str`类型不能可变的索引 
  // s.push('\n'); 
}