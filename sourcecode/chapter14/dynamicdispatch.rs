trait Processor { 
  fn compute(&self, x: i64, y: i64) -> i64; 
  } 
  struct Risc {} 
  impl Processor for Risc { 
  fn compute(&self, x: i64, y: i64) -> i64 { 
  x + y 
  } 
  } 
  struct Cisc {} 
  impl Processor for Cisc { 
  fn compute(&self, x: i64, y: i64) -> i64 { 
  x * y 
  } 
  } 
  fn process(processor: &dyn Processor, x: i64) { 
  let result = processor.compute(x, 42); 	// 具体执行哪个对象的compute函数，由processor对象决定
  println!("{}", result); 
  } 
  pub fn main() { 
  let processors: Vec<Box<dyn Processor>> = vec![ 
  Box::new(Cisc{}), 
  Box::new(Risc{}), 
  ]; 
  for processor in processors { 
  process(&*processor, 1); 	// process函数具体调用CISC还是RISC，由processor对象类型决定
  } 
  } 