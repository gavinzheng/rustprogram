// #![feature(dyn_trait)]
trait Foo {
 fn foo1(&self);
 fn foo2(&self) where Self: Sized;
}
impl Foo for i32 {
 fn foo1(&self) { println!("foo1 {}", self); }
 fn foo2(&self) where Self: Sized { println!("foo2 {}", self); }
}
fn main() {
 let x = 1_i32;
 x.foo2();
 let p = &x as &dyn Foo;
 p.foo2();
}