fn main() { 
  let val = Box::new(vec![1,2,3]); 
  // 借助Deref，我们仍然可以使用向量的方法，而不用对Box进行处理 
  val.iter().fold(0, |acc, &x| acc + x ); // 6 
  // 我们传给函数指向Vec的Box指针，Box<Vec>自动强制解引用为Vec 
  f(&val) 
  } 
  fn f(x: &Vec<i32>) { 
  println!("{:?}", x) // [1,2,3] 
  } 