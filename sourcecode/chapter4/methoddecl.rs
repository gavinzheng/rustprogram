struct Circle { 
x: f64, 
y: f64, 
radius: f64, 
}
impl Circle { 
fn area(&self) -> f64 { 
std::f64::consts::PI * (self.radius * self.radius) 
}
fn grow(&self, increment: f64) -> Circle { 
Circle { x: self.x, y: self.y, radius: self.radius + increment } 
} 
}
fn main() { 
let c = Circle { x: 0.0, y: 0.0, radius: 2.0 }; 
println!("{}", c.area()); 
let d = c.grow(2.0).area(); 	// 方法的链式调用
println!("{}", d); 
} 