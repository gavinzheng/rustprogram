#[derive(Default,Debug)] 
struct ColorRGB{ 
  r: i32, 
  g: i32, 
  b: i32, 
}
fn main() {
  // 可以使用default()函数初始化其他的元素。 关于default trait，请参考14.3.2节
  // ..expr 这样的语法,只能放在初始化表达式中,所有成员的最后，且最多只能有一个 
  let origin = ColorRGB::default(); 		// 赋予origin默认值	
  let point = ColorRGB{ g: 100, ..origin }; 	// point变量的g为100，其他字段的值来自origin的相应字段
  let ColorRGB{ r: r0, g: g0, .. } = point;

  println!("origin {:?} ", origin);
  println!("point {:?} ", point);
  println!("{} {}",r0, g0);
}