#[allow(unused_variables)]

const MAX_LENGTH: &'static i32 = &10; 
static COMPANY_NAME: &'static str = "BEIKE XinChain";

fn main(){
  let heap_vec1: Vec<i32> = vec![10,11,12];
  let heap_vec2 = heap_vec1 ;  // 通过赋值，所有权转移给heap_vec2
  // println!("Heap_vec1 content is {}", heap_vec1.len()); // 触发编译错误，因为heap_vec1不再存在
  println!("Heap_vec2 content is {}", heap_vec2.len());  // 成功
}