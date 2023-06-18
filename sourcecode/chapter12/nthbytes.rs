
fn print_codes(s: &str) {
  let mut iter = s.chars();
  loop {
    match iter.next() {
      Some(c) => { println!("{}: {}", c, c as u32); },
      None => { break; },
    }
  }
 }

fn main(){
  let s = "abc012è€测试";
  for i in 0..s.len() {
   println!("{}: {}", i, s.as_bytes()[i]);
  }
  print_codes("è测试");
}
