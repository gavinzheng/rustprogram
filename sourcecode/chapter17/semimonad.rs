fn main() 
{ 
  let v1 = Some(2).and_then(|x| Some(x+x)).and_then(|y| Some(y*y)); 
  println!("{:?}", v1); 
  let v2 = None.or_else(|| None).or_else(|| Some(222)); 
  println!("{:?}", v2); 
} 