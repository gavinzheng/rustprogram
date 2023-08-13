use std::collections::HashMap; 
#[derive(Clone, Hash, PartialEq, Eq)] // 实现Hash和Eq,才能作为HashMap的键
struct SaleData { 
member1: String, 
member2: Vec<u32>, 
member3: i32, 
} 
fn main() { 
let key1 = SaleData { 
member1: "Field1".to_owned(), 
member2: vec![0, 10, 20], 
member3: 1000,
}; 
let key2 = key1.clone(); 			// key1 == key2
let key3 = SaleData { 
member1: "Field2".to_owned(), 
member2: vec![15, 11, 3], 
member3: 2000, 
}; 
let mut map = HashMap::new(); 
map.insert(key1, "FirstSale"); 		// 以key1为键插入FirstSale值
assert!(map.get(&key2).is_some()); 	// 因为key1==key2，所以get会返回值
assert!(map.get(&key3).is_none()); 	// 因为key3不存在，所以get会返回none
} 