fn main(){
  let taoism= "大智若愚"; 
  for b in taoism.as_bytes() { 
    print!("{}, ", b); 
  }
  println!(""); 
  for c in taoism.chars() { 
    print!("{}, ", c); 
  }
  println!(""); 
  
  assert_eq!("瑾瑜".len(), 6); 
  assert_eq!("怀瑾握瑜".chars().count(), 4); 
}