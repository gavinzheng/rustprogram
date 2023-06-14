#![feature(nll)] 

use std::collections::HashMap; 

fn example(map: &mut HashMap<i32, i32>, key: i32) 
{ 
  match map.get_mut(&key) { 
    Some(value) => println!("{}", value), 
    None => { 
      map.insert(key, 0); 
    } 
  } 
} 
fn main() { 
  let mut map = HashMap::<i32, i32>::new(); 
  let key = 100; 
  example(&mut map, key); 
} 