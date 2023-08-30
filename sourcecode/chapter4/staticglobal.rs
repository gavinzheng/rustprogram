fn main() {
  println!("Number = {}", MY_STATIC);
}
static MY_STATIC: u32 = foo();
fn foo() -> u32 {
  42				// 运行时获取的值，比如当前内存，磁盘大小等
}