fn main() {
  let o1 = Box::new("Milly");
  let o2 = o1;                // <-- o1 is 'moved' into o2 and now invalid
  println!("Hello, {}!", o1); // <-- this is a compile time error
}