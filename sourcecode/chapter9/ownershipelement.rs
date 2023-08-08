#[allow(unused_variables)]
fn main(){
  let mut v = Vec::new(); 
  for i in 10 .. 16 { 
  v.push(i.to_string()); 
  } 
  // 取出第3个元素和第6个元素
  let ele3 = v[2]; 
  let ele10 = v[5]; 
}