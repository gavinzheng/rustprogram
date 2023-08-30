fn main() {
  let mut squares = (0..10).map(|i| i*i); 
  assert_eq!(squares.nth(4), Some(16)); 
  println!("001- {:?}", squares);
  assert_eq!(squares.nth(0), Some(25)); 
  println!("002- {:?}", squares);
  assert_eq!(squares.nth(2), Some(64));
  println!("003- {:?}", squares);
  assert_eq!(squares.nth(6), None);
}