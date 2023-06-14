
fn main() {
  let mut num = 5; 
  { 
    let mut add_num = move |x: i32| num += x; // 通过借用方式
    add_num(5); 
  } 
  assert_eq!(10, num);
}