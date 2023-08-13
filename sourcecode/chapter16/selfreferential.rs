#[derive(Debug)]
struct Test {    
  a: String,    
  b: *const String, // 改成指针
}
impl Test {    
  fn new(txt: &str) -> Self {        
      Test {            
        a: String::from(txt),            
        b: std::ptr::null(),        
      }    
  }
  fn init(&mut self) {        
    let self_ref: *const String = &self.a;        
    self.b = self_ref;    
  }
  fn a(&self) -> &str {  &self.a    }
  fn b(&self) -> &String {   unsafe {&*(self.b)}    }
}
fn main() {    
  let mut step1 = Test::new("step1");    
  step1.init();    
  let mut step2 = Test::new("step2");    
  step2.init();
        println!("a: {}, b: {}", step1.a(), step1.b());    // 使用swap()函数交换两者，这里发生了move    
  std::mem::swap(&mut step1, &mut step2);   
  step1.a = "现在已经改变!".to_string();    	   
  println!("a: {}, b: {}", step2.a(), step2.b());
}