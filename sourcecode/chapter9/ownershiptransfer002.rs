#[allow(unused_variables)]
fn main(){
  // let stack_i32: i32 = 1000;
  let heap_i32: Box<i32> = Box::new(200) ;

  stack_procedure(stack_i32);
  println!("In main stack_i32 {}", stack_i32); 

  heap_i32 = heap_procedure(heap_i32);
  println!("In main heap_i32 is {}", heap_i32);  // 成功
}
fn stack_procedure(mut param: i32){
  param += 100;
  println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: Box<i32> ) -> i32{
  println!("In heap_procedure with param ");
  param
}