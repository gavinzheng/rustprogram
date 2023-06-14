trait Trait {}
fn aFunc<X: Trait>(t: X) {}
impl<'a> Trait for &'a i32 {}		// 为i32实现了Trait
fn main() {
    let p: &mut i32 = &mut 0;
    aFunc(p);
}