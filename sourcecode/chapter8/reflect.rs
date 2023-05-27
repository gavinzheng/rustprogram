use std::any::Any; 
use std::fmt::Debug ; 
fn readconfig<T:Any+Debug>(value: &T) -> Vec<String>{ 
  let mut arrcfg: Vec<String>= vec![]; 
  let value = value as &dyn Any; 
  match value.downcast_ref::<String>() { 
    Some(para) => arrcfg.push(para.clone()), 
    None => (), 
  }; 
  match value.downcast_ref::<Vec<String>>() { 
    Some(v) => arrcfg.extend_from_slice(&v), 
    None =>(), 
  } 
  if arrcfg.len() == 0 { 
    panic!("Config does not exist"); 
  } 
  arrcfg 
} 
fn main() { 
  let para1 = "systemlog.conf".to_string(); 
  assert_eq!(readconfig(&para1), vec!["systemlog.conf".to_string()]); 
  let vecpara = vec!["systemlog.conf".to_string(), "systemlog2.conf".to_string()]; 
  assert_eq!(readconfig(&vecpara), 	vec!["systemlog.conf".to_string(), "systemlog2.conf".to_string()]); 
} 