fn main () { 
  let v1 = 4u8..10u8; 
  let v2 = 4u8..10; 
  let v3 = 4..10u8; 
  let v4 = 4..10; 
  let v5 = -4..10; 
  let v6 = 4..10 as i64; 
  print!( "{} {} {} {} {} {}", std::mem::size_of_val(&v1), std::mem::size_of_val(&v2), 
  std::mem::size_of_val(&v3), std::mem::size_of_val(&v4),std::mem::size_of_val(&v5), 
  std::mem::size_of_val(&v6)); 
}