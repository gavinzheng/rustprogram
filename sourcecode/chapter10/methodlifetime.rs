struct foo(i32); 
impl foo{ 
// 标注生命周期，就像独立的函数一样。 
fn add_10<'a>(&'a mut self) { self.0 += 10; } 
fn print<'a>(&'a self) { 
println!("`print`: {}", self.0); 
} 
} 
fn main() { 
let mut instance= foo(18); 
instance.add_10(); 
instance.print(); 
} 