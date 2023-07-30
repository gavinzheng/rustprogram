extern crate lazy_static;
use lazy_static::lazy_static;

fn main() {
  println!("Number = {}", *MY_STATIC);	// 使用deref trait
}
lazy_static! {
  static ref MY_STATIC: u32 = foo();		// 是引用类型
}
fn foo() -> u32 {
    	42
}