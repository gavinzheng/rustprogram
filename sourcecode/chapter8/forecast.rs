trait SomeTrait  {}
fn aFunc<X: SomeTrait >(t: X) {}
impl<'a> SomeTrait  for &'a i32 {}		// SomeTrait 
fn main() {
    let p: &mut i32 = &mut 0;
    aFunc(p);
}