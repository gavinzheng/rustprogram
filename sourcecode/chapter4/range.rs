fn main () { 
  assert_eq!((10.. 15) , std::ops::Range{ start: 10 , end : 15 }); //左闭右开
  assert_eq!((10..=15) , std::ops::RangeInclusive::new(10, 15)); //全闭
  assert_eq!(3+4+5 , (3..6).sum()); 
  assert_eq!(3+4+5+6, (3..=6).sum()); 
  
  for l in 10..15{ 
      println!("{}",l); //  输出为1 , 2 , 3 , 4 
  }
  for i in 10..=15  { 
      println!("{}", i); // 输出为1 , 2 , 3 ,4,5  
  }
  //assert_eq!((=l.. 5) , std::ops::Range{ start: 1 , end : 5 }); //左闭右开
}