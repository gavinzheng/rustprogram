fn longest<'a>(x: &str, y: &str) -> &'a str {
  let result = String::from("really long string");
  result.as_str()
}
fn main() {
  let s; 
  { 
  s = longest("1234","abcde"); 
  } 
  println!("Smallest String is {}", s); 
}