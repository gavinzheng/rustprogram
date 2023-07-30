fn main() -> Result<(), &'static str> { 
  let s = vec!["coke", "7up"]; 
  let third = s.get(3).ok_or("I got only 2 drinks")?; 
  Ok(()) 
  }