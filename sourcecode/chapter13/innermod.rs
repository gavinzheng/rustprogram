mod sports { 
  pub struct Football; 
  struct Basketball; 
  struct Volleyball; 
} 

// use sports::football; 
//use crate::sports::football;
//use self::sports::Football;
use self::sports::*;

fn main() { 
  let playing = Football; 
  // let playing = Volleyball;
}