fn main() { 
  let inf = std::f32::INFINITY;
  println!("{} {} {}", inf * 0.0, 1.0 / inf, inf / inf);
  let x = (-33.0_f32).sqrt(); 
  assert_eq!(x, x); 
}