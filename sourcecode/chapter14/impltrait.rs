trait Animal {
  fn talk(&self);
}
struct Cat {}
struct Dog {}
impl Animal for Cat {
  fn talk(&self) {
      println!("meow");
  }
}
impl Animal for Dog {
  fn talk(&self) {
     println!("bark");
  }
}
fn animal_talk(a: &dyn Animal) {
   a.talk();
}
fn main() {
   let d = Dog {};
   let c = Cat {};  animal_talk(&d);
   animal_talk(&c);
}