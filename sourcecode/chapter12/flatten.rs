fn main() { 
  let x = vec![vec![10, 20, 30, 40, 50], vec![60, 70, 80, 90]].into_iter();  // 二维数据
  let z: Vec<u64> = x.flatten().collect(); 	// 二维数据编程一维数据
  assert_eq!(z.len(), 10); 
  } 