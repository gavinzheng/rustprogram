 
fn main() { 
  let a: String = "Hello".to_string(); 
  let b = String::from("Hello"); 
  let c = "Milly".to_owned(); 
  let d = c.clone(); 
  let e = format!("{}°{:02}'{:02}″N", 24, 5, 23);
  let bits = vec!["veni", "vidi", "vici"]; 
  assert_eq!(bits.concat(), "venividivici"); 
  assert_eq!(bits.join(", "), "veni, vidi, vici");

  println!("{} {} {} {} {}", a,b,c,d,e); 
}