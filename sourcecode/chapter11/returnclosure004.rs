fn getclosure() -> &'static (Fn(i32) -> i32) { 
  let num = 10; 
  |x| x + num 
} 
fn main(){
  let f = getclosure(); 
  let answer = f(2); 
  assert_eq!(12, answer); 
}
  