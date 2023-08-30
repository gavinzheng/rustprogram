fn main() {
  let v = vec!["张三".to_string(),"李四".to_string(),"王五".to_string()]; 
  for mut s in v { 
    s.push('?'); 
    println!("{}", s); 
  } 
}