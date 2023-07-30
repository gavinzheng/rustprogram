#[derive(Default,Debug)] 
struct ColorRGB{ 
 r: i32, 
 g: i32, 
 b: i32, 
}
fn main() { 
  let init = ColorRGB{ r: 0, g: 0, b:0}; 
  // r0,g0和b0 ,分别绑定到成员 r,g 和成员 b 
  let ColorRGB{ r : r0, g : g0, b:b0 } = init ; 
  println!("ColorRGB 是 {} {} {}", r0, g0, b0);  // r0, g0, b0来自init相应的字段 
  
  // 同理,在模式匹配的时候,如果新的变量名刚好和成员名字相同,可以使用简写方式 
  let ColorRGB{ r, g, b } = init; 	// 解构init结构为r,g,b
  println!("ColorRGB 是 {} {} {}", r, g, b); 
  } 