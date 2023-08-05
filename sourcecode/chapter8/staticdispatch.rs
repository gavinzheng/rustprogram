use std::ops::Add; 
fn sd_add<T: Add<Output=T>>(x: T, y: T) -> T { // Output为关联类型
x + y 
} 
fn main() { 
assert_eq!(48, sd_add(16 as u8, 32 as u8)); 	// 编译时生成一个u8类型的sd_add()
assert_eq!(48, sd_add(16 as u64, 32 as u64)); 	// 编译时生成一个u64类型的sd_add()
}