fn main() { 
  let v = vec![56]; 
  if v[0] == 56 { 
  let s = String::from("hello milly"); 
  println!("string is {}; vector is {:?}", s, v); 
  } // s 作⽤域结束，String析构
  } // v 作⽤域结束，vec析构