use std::ops::Deref; 
struct Student<T> { 
value: T, 
} 
impl<T> Deref for Student<T> { 
type Target = T; 		// 关联类型Target，代表解引用之后的目标类型。

fn deref(&self) -> &T { 
&self.value 
} 
} 
fn main() { 
let x = Student{ value: "Alice" }; 
assert_eq!("Alice", *x); 
} 