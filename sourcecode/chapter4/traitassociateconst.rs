trait Circular { 
  const PI: f64 = 3.14; 		// 在trait里定义常量
  fn calarea(&self) -> f64; 
} 
struct Circle { 
  radius: f64 
} 
impl Circular for Circle { 
  fn calarea(&self) -> f64 { 
    Circle::PI * self.radius* self.radius	// 使用Circle::PI使用该常量
  } 
} 
fn main() { 
  let c1 = Circle { radius: 5.6 }; 
  let c2 = Circle { radius: 68.5 }; 
  println!("Area of circle one: {}", c1.calarea()); 
  println!("Area of circle two: {}", c2.calarea()); 
} 