fn main(){
  let arr = [11, 22, 33, 44]; 
  let n = 2; 
  let sr1 = &arr[..n];  	// 等同于 &arr[0..n]
  let sr2 = &arr[n..]; 	// 等同于 &arr[n..arr.len()]
  println!("{:?} {:?}", sr1, sr2); 

  let v1: std::ops::RangeFrom<i32> = 3..; 
  let v2: std::ops::RangeTo<i32> = ..12; 
  println!("{:?} {:?} {} {}", v1, v2, std::mem::size_of_val(&v1),std::mem::size_of_val(&v2)); 
  println!("RangeFrom: {}  Rangeto: {} ", 
    std::mem::size_of::<std::ops::RangeFrom<i32>> as u64,
    std::mem::size_of::<std::ops::RangeTo<i32>> as u64);

  for i in 4.. { 					      // RangeFrom类型
    if i * 10 > 100 { break; } 	// 条件break
    println!("{} ", i); 
  } 

  let range: std::ops::RangeFull = ..; 	// RangeFull类型
  let a1 = [11, 22, 33, 44]; 
  let a2 = &a1[range]; 
  print!("{} {:?} {:?}", std::mem::size_of_val(&range), a1, a2); 
}