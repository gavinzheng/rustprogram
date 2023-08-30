use std::fmt::{Display, Formatter, Error}; 
#[derive(Debug)] 
struct T { 
field1: i32, 
field2: i32, 
} 
impl Display for T { 
fn fmt(&self, f: &mut Formatter) -> Result<(), Error> { 
write!(f, "{{ field1:{}, field2:{} }}", self.field1, self.field2) 
} 
} 
fn main() { 
let var = T { field1: 1, field2: 2 }; 
println!("{}", var); 
println!("{:?}", var); 
println!("{:#?}", var); 
} 