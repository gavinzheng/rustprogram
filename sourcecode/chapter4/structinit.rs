struct Color{
  red : i32,
  green : i32,
  blue :i32,
  bright : f32,
  }
  fn main() { 
  // 刚好局部变量名字和结构体成员名字一致 
  let red = 10; 		// 变量名和Color结构中的red字段同名 
  let green = 200; 	// 变量名和Color结构中的green字段同名
  
  // 下面是简略写法,等同于 Color{ red: red, green: green, blue : 55, bright: 20.0},同名字的相对应
  // red，green由于名字一样，所以冒号前字段名可以省略， 不用写成red：red， green:green
  let c = Color{ red , green, blue: 55, bright : 20.0 }; 
  println!("Color 是 {} {} {}", c.red, c.green, c.blue); 
  } 