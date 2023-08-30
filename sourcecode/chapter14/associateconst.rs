//#![feature(associated_consts)] 
trait Foo { 
  const ID: i32; 
} 
impl Foo for i32 { 
  //const ID: i32 = 1;  // 必须要实现，否则就是未初始化的值
} 
impl Foo for i64 { 
  //const ID: i32 = 5; 
} 
fn main() { 
  assert_eq!(1, i32::ID); 
  assert_eq!(5, i64::ID); 
}