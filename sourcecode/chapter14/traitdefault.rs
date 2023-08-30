pub trait traitdemo { 
  fn hello(&self) -> String {   // Hello trait中hello()函数的默认实现 
    String::from("World") 
  } 
}
  pub struct Milly{} 
  impl traitdemo for Milly{ 
    fn hello(&self) -> String { // Milly trait实现了hello()方法
      String::from("Milly") 
    } 
  }
   
  pub struct Anonymous {} 
  impl traitdemo for Anonymous {}   // impl块中并没有hello()方法的实现
  impl traitdemo for Vec::<i32> {} 
  
  fn main() { 
    let milly= Milly{}; 
    let anonymous = Anonymous{}; 
    println!("Milly: {}", milly.hello()); 
    println!("Anonymous: {}", anonymous.hello()); 	// 此处调用Hello trait中的默认hello()方法
  } 