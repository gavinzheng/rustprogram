use lazy_static::lazy_static;

lazy_static! {
  static ref MY_STATIC: u32 = foo();
}

fn main() {
    println!("Number = {}", *MY_STATIC);
}

fn foo() -> u32 {
  42
}
