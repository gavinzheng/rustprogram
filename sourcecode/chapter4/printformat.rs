use std::f64::consts::PI;

fn main(){ 
  // 打印精确到小数点后2位 
  println!("{:.2}",1.2345 ); 
  println!("================"); 
  // 以2进制，16进制和八进制方式打印
  println!("B: {:b} H: {:x} O: {:o}",10,10,10 ); 
  println!("================"); 
  // Shifts 
  println!("{ten:>0ws$}",ten=10, ws=5 ); 
  println!("{ws:>0ten$}",ten=10, ws=5 ); 
  println!("pi is {:e} in floating point notation", PI); // 3.14e0 

  println!("{:?}", Some("fantastic"));
  println!("{:#?}", vec![Some("Hello"), None, Some("World")]);
  
  // You can specify the position of arguments using numerical indexes. 
  println!("{1} {0}", "World", "Hello"); 
  // You can use named arguments with format 
  println!("{greeting} {who}!", greeting="Hello", who="World"); 
  // You can mix Debug and Display prints: 
  println!("{greeting} {1:?}, {0}", "and welcome", Some(42), greeting="Hello"); 
} 