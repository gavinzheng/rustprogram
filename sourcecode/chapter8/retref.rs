
fn min(v: &[i32]) -> &i32 { 
  let mut s = &v[0]; 
  for r in &v[1..] { 
  if *r < *s { s = r; } 
  } 
  s 
}
fn main() {
  let s; 
  { // 块代码开始
    let arr= [10, 5, 6, 1, 9, 7, 8]; 
    s = min(&arr); 
  } // 块代码结束
  assert_eq!(*s, 0); // s所指向的对象已经被Drop
}