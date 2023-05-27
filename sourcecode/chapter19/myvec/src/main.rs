// #![feature(box_syntax)]

macro_rules! my_vec1 { 
  ($($x: expr),*) => {{ 
    let mut vector = Vec::new(); 
    $(vector.push($x);)* 
    vector 
  }} 
} 
macro_rules! my_vec2 { 
  ($elem:expr; $n:expr) => ( 		// 匹配分支1 
        $crate::vec::from_elem($elem, $n) 
      ); 
  ($($x:expr),*) => ( 			//  匹配分支2 - 匹配词条序列，比如： 1, 2, 3. 
        <[_]>::into_vec(Box::new [$($x),*]) 
      ); 
  ($($x:expr,)*) => (vec![$($x),*]) //  匹配分支3 -匹配词条序列，比如： 1, 2, 3，vec![ 1,2,3,]; 
} 
macro_rules! my_vec3 { 
  ($elem:expr; $n:expr) => ( 		// 匹配分支1 ，比如[10;5] 即值为10的5个元素的矢量
    $crate::vec::from_elem($elem, $n) 
  ); 
  ($($x:expr),*) => ({ 			// 匹配分支2
      let mut v = Vec::new(); 
      $(v.push($x);)* 
      v 
    }); 
  ($($x:expr,)*) => (vec![$($x),*]) 	// 匹配分支3
} 
fn main() { 
  let x = my_vec1![1, 2, 3, 4]; 
  let y = my_vec2![1, 2, 3, 4]; 
  let z = my_vec3![1, 2, 3, 4]; 

  let xx = vec![1, 2, 3, 4,]; 
}