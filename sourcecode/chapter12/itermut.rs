fn main(){
  let slice1 = &[3, 4, 5]; 
  let slice2 = &[7, 8]; 
  let mut iterator = slice1.iter(); 	// 可变迭代子
  for item_ref in iterator { 
  print!("[{}] ", *item_ref); 
  }
  iterator = slice2.iter(); 			// 被赋予了一个不可变的迭代子值
  for item_ref in iterator { 
  print!("({}) ", *item_ref); 
  }
}