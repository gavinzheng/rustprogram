#[allow(unused_variables)]

fn main(){
  let heap_vec = vec![100,200,300];						// 堆变量
  let heap_i32: Box<i32> = Box::new(200) ;		// 堆变量

  heap_procedure1(&heap_vec);		// Coord已经实现了Copy和Clone trait，这里传的是拷贝
  println!("In main stack_i32 {}", heap_vec[0]);  	// OK

  heap_procedure2(&heap_i32);	// 堆变量通过clone函数生成拷贝，传给调用函数。heap_i32仍可用
  println!("In main heap_i32 is {}", heap_i32); // OK 编译成功
}
fn heap_procedure1(param: &Vec<i32>){
   println!("In heap_procedure1 with param ");
}

fn heap_procedure2(param: &Box<i32> ){	// 拷贝转移所有权给param，函数返回时，param被释放
  println!("In heap_procedure2 with param {}",*param);
}