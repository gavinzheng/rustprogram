use std::convert::From;

#[derive(Debug)]
struct Number { num: i32}
impl From<i32> for Number {		// 为Number实现From<i32> trait
    fn from(item: i32) -> Self {	// 实现From<i32>的方法
        Number {num: item}
    }
}
fn main() {
    let i = 100;
    let var1= Number::from(i);		// var1是Number类型：i32转换为Number类型
    let var2: Number = i.into();		// var2是Number类型：i32转换为Number类型
    println!("var1: {:?}, var2: {:?}", var1, var2);
}