use std::num::Wrapping; 
fn main() { 
  let big = Wrapping(std::u32::MAX); 
  let sum = big + Wrapping(2_u32); 
  println!("{}", sum.0); 

  let big_val = std::i32::MAX; 
  //let x = big_val + 1; // 算术操作溢出，引发panic 
  let x = big_val.wrapping_add(1); // ok 
} 