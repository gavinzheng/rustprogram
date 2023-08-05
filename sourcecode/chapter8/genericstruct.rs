struct Node<T=i32> { 		// 此处省略了指针域，默认T类型为i32,如果没有指定，T就是i32
  item: T 
  } 
  impl<T> Node<T> { 		// Node<T>的impl块
  fn new(item: T) -> Self { 
  Node{ item } 
  } 
  } 
  fn main() { 
  let v1 = Node { item: 0}; 				// 没有显式指定T,则T为默认类型i32
  let v2 = Node::<bool> { item: false};		// 显式指定T为bool类型，则T为bool类型
  }