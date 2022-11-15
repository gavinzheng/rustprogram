#[allow(unused_variables)]
fn main(){
 
  let mut heap_i32: Box<i32> = Box::new(200) ;

  heap_i32 = heap_procedure(heap_i32);
  println!("In main heap_i32 is {}", heap_i32);  // 成功
}

fn heap_procedure(param: Box<i32> ) -> Box<i32>{
  println!("In heap_procedure with param ");
  param
}