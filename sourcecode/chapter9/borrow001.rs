fn main() {
  let s = String::from("MIlly");
  let ref1: &String = &s;
  let ref2: &String = ref1;
  println!("{}\n{}", ref1,ref2);
}