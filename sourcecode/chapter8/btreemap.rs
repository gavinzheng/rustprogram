use std::collections::BTreeMap; 
fn main() { 
  let mut map = BTreeMap::new(); 
  map.insert(31, "Alice"); 
  map.insert(56, "Milly"); 
  map.insert(87, "北科软件"); 
  for (k, v) in map.range(20..60) { 
    println!("{} : {}", k, v); 
  } 
}