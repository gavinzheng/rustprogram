
// fn square(x: i32) -> NumberwithLogs{
//   // return x * x 
//   return NumberwithLogs {result: x*x, logs: vec!["Squraed x to get x*x".to_string()]}
// }

fn square(x: &NumberwithLogs) -> NumberwithLogs{
  // return x * x 
  return NumberwithLogs {
    result: x.result*x.result, 
    logs: vec!["Squraed x to get x*x".to_string()] // { let mut vec = x.logs.clone(); vec.push("Squraed x to get x*x".to_string()); println!("{:?}",vec); vec}
  }
}

// fn addOne(x:i32) ->i32{
//   return x +1 
// }
fn addOne(x: &NumberwithLogs) -> NumberwithLogs{
  // println!("Added 1 to get x*x");
  return NumberwithLogs {result: x.result + 1, logs: vec!["Added 1 to get x+1".to_string()]}
}

fn wrapwithLogs(x:i32)->NumberwithLogs{
  return NumberwithLogs{result: x, logs:vec![]}
}
struct NumberwithLogs{
  result: i32,
  logs:  Vec<String>,
} 

fn runwithlogs(input: NumberwithLogs, transform : &dyn Fn(&NumberwithLogs) -> NumberwithLogs) -> NumberwithLogs{
  let mut newNumberWithLogs = transform(&input);
  return NumberwithLogs{
    result: newNumberWithLogs.result, 
    logs: { let mut vec = input.logs.clone(); vec.extend(newNumberWithLogs.logs); println!("{:?}",vec);vec}}
}

fn main(){
  // addOne(square(wrapwithLogs(2)));
  let a = runwithlogs(wrapwithLogs(5),&addOne);
  let b = runwithlogs(a,&square);
}