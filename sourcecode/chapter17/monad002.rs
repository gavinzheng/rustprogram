
// fn square(x: i32) -> NumberwithLogs{
//   // return x * x 
//   return NumberwithLogs {result: x*x, logs: vec!["Squraed x to get x*x".to_string()]}
// }

fn square(x: &mut NumberwithLogs) -> NumberwithLogs{
  // return x * x 
  return NumberwithLogs {
    result: x.result*x.result, 
    logs: x.logs.clone(), // .append(vec!["Squraed x to get x*x".to_string()])
  }
}

fn addOne(x:NumberwithLogs) -> NumberwithLogs{
  return NumberwithLogs {result: x.result + 1, logs: vec!["Added 1 to get x*x".to_string()]}
}

fn wrapwithLogs(x:i32)->NumberwithLogs{
  return NumberwithLogs{result: x, logs:vec![]}
}
struct NumberwithLogs{
  result: i32,
  logs:  Vec<String>,
} 

fn runwithlogs(input: NumberwithLogs, transform : &dyn Fn(i32) -> NumberwithLogs) -> NumberwithLogs{
  let mut newNumberWithLogs = transform(input.result);
  return NumberwithLogs{result: newNumberWithLogs.result, logs: input.logs.extend(newNumberWithLogs.logs)}
}

fn main(){
  addOne(square(wrapwithLogs(2)));
}