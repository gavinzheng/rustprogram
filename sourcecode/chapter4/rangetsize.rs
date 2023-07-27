fn main () { 
  let range1 = 5u8..20u8; 
  let range2 = 5u8..20; 
  let range3 = 5..20u8; 
  let range4 = 5..20; 
  let range5 = -5..20; 
  let range6 = 5..20 as i64; 
  println!( "{} {} {} {} {} {}", 
  std::mem::size_of_val(&range1), 
  std::mem::size_of_val(&range2), 
  std::mem::size_of_val(&range3), 
  std::mem::size_of_val(&range4), 
  std::mem::size_of_val(&range5), 
  std::mem::size_of_val(&range6));
}