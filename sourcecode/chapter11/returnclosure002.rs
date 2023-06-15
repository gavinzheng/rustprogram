fn getclosure() -> Box<Fn(i32) -> i32> { 
  let num = 10; 
  Box::new(move |x| x + num) 
} 
fn main() { 
  let f = getclosure(); 
  let answer = f(2); 
  assert_eq!(12, answer); 
} 