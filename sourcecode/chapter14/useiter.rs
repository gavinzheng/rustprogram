use std::iter::Iterator; 
use std::fmt::Debug; 
fn use_iter<ITEM, ITER>(mut iter: ITER)  // 其中ITEM是关联类型，ITER为泛型参数
  where ITER: Iterator<Item=ITEM>, 	  // Iterator的关联类型Item指定为ITEM 	
ITEM: Debug 					  // 关联类型必须支持Debug trait
{ 
  while let Some(i) = iter.next() { 
    println!("{:?}", i); 
  } 
} 
fn main() { 
  let v: Vec<i32> = vec![1,2,3,4,5]; 
  use_iter(v.iter()); 
} 