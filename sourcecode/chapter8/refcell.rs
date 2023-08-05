use std::cell::RefCell; 
struct Foo { 
  number: u8 
} 
fn main() { 
  let foo1 = RefCell::new(Foo { number: 1 }); 
  let mut ref_foo_1 = foo1.borrow_mut();// 连续2个可变借用会触发编译错误-任何时候只能有一个可变引用
  let mut ref_foo_2 = foo1.borrow_mut(); 
  ref_foo_1.number = 2; 
  ref_foo_2.number = 3; 
}