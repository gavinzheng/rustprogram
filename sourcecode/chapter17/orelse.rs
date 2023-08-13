fn main() { 
  let v3 = Some(10).and_then(|x| Some("ABC")); 
  println!("{:?}", v3); 
  // or_else不是标准意义上的monad ，因为它不允许多态的bind
  //let v4 = Some(2).or_else(|| Some("")); 
  //println!("{:?}", v4); 
  }