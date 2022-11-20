
fn main() {
  let mut v = Vec::new(); 
  for i in 10 .. 16 { 
    v.push(i.to_string()); 
  } 
  // 1. 将vector顶部的元素推出（此处顶部是第10个元素）: 
  let ele5 = v.pop().unwrap(); 
  assert_eq!(ele5, "15"); 
  // 2. 把元素移出向量，并将向量的最后一个元素拿来填补其空位
  let ele2 = v.swap_remove(1); 
  assert_eq!(ele2, "11"); 
  // 3. v[2]被”new”替换，被替换的值在返回值中 
  let ele3 = std::mem::replace(&mut v[2], "new".to_string()); 
  assert_eq!(ele3, "12"); 
  // 最终向量的情况
  assert_eq!(v, vec!["10", "14", "new", "13"]);
}