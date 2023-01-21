mod sports { 
  pub struct Football; 
  struct Basketball; 
  struct Volleyball; 
} 

// use sports::football; 
//use crate::sports::football;
use self::sports::Football;

fn main() { 
  let playing = Football; 
}