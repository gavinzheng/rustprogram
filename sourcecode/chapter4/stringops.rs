fn main() {
  assert!("peanut".contains("nut")); 
  assert_eq!("哈_哈".replace("哈" ,"呵"),"呵_呵");
  assert_eq!(" clean\n".trim(), "clean"); 
  for word in "veni, vidi, vici".split(", ") { 
    assert!(word.starts_with("v")); 
  } 
}