use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::cell::Cell;
#[derive(Eq, PartialEq)]
struct BadKey {
 value: Cell<i32>
}
impl BadKey {
 fn new(v: i32) -> Self {
 BadKey { value: Cell::new(v) }
 }
}
impl Hash for BadKey {
 fn hash<H: Hasher>(&self, state: &mut H) {
 self.value.get().hash(state);
 }
}
fn main() {
 let mut map = HashMap::new();
 map.insert(BadKey::new(1), 100);
 map.insert(BadKey::new(2), 200);
 for key in map.keys() {
 key.value.set(key.value.get() * 2);
 }
 println!("Find key 1:{:?}", map.get(&BadKey::new(1)));
 println!("Find key 2:{:?}", map.get(&BadKey::new(2)));
 println!("Find key 4:{:?}", map.get(&BadKey::new(4)));
}