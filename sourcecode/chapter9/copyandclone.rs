#[allow(unused_variables)]
#[derive(Debug,Copy,Clone)]
struct Coord{ x:i32, y: i32 } 

fn main(){
  let stack_struct =  Coord{x:100,y:100};						// 栈结构变量
  let heap_i32: Box<i32> = Box::new(200) ;		// 堆变量

  stack_procedure(stack_struct);		// Coord已经实现了Copy和Clone trait，这里传的是拷贝
  println!("In main stack_i32 {}", stack_struct.x);  	// OK

  heap_procedure(heap_i32.clone());	// 堆变量通过clone函数生成拷贝，传给调用函数。heap_i32仍可用
  println!("In main heap_i32 is {}", heap_i32); // OK 编译成功
}
fn stack_procedure(mut param: Coord){
  param.x += 100;
  println!("In stack_procedure with param ");
}

fn heap_procedure(param: Box<i32> ){	// 拷贝转移所有权给param，函数返回时，param被释放
  println!("In heap_procedure with param ");
}