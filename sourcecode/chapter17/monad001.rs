
fn square(x: i32) -> i32 { //NumberwithLogs{
  return x * x 
  // return NumberwithLogs {result: x*x, logs: vec!["Squraed x to get x*x".to_string()]}
}

fn addOne(x:i32) ->i32{
  return x +1 
}

fn wrapwithLogs(x:i32)->NumberwithLogs{
  return NumberwithLogs{result: x, logs:vec![]}
}

struct NumberwithLogs{
  result: i32,
  logs:  Vec<String>,
} 

fn main(){
  addOne(square(2));
}