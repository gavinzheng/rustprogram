fn main () { 
  assert_eq!((1.. 5) , std::ops::Range{ start: 1 , end : 5 }); //左闭右开
  assert_eq!((1..=5) , std::ops:Rangeinclusive::new(1, 5)); //全闭
  assert_eq!(3+4+5 , (3..6).sum()); 
  assert_eq!(3+4+5+6, (3..=6).sum()); 
  
  for l in (1..5) { 
      println!("{}",l); //  输出为1 , 2 , 3 , 4 
  }
  for i in (1..=5 ) { 
      println!("{}", i); // 输出为1 , 2 , 3 ,4,5  
  }
  //assert_eq!((=l.. 5) , std::ops::Range{ start: 1 , end : 5 }); //左闭右开
}