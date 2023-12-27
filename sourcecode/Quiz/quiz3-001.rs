struct S<'a, 'b>(&'a u8, &'b u8);
enum E<'a, 'b> {
  V(&'a u8),
  U(&'b u8),
}
fn main() {
  S(&0, &0); // OK
  S::<'static, 'static>(&0, &0);

  E::V(&0); // OK
  E::V::<'static, 'static>(&0);
}