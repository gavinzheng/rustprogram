use std::ops::Drop;
struct T;
impl Drop for T {
 fn drop(&mut self){}
}
impl Copy for T {}
fn main() {}