use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

#[derive(Clone)]
struct MyHasher {
    count: u64,
}

impl Hasher for MyHasher {
    fn finish(&self) -> u64 {
        self.count
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.count = self.count.wrapping_add(*byte as u64);
        }
        // println!("count: {}", self.count);
    }
}

impl BuildHasher for MyHasher {
    type Hasher = Self;
    fn build_hasher(&self) -> Self::Hasher {
        self.clone()
    }
}

fn main() {
    // 仅用于示例：<u8, u8> 
    let map: HashMap<u8, u8> = HashMap::with_hasher(RandomState::new());

    let mut map = HashMap::with_hasher(MyHasher { count: 56789 });
    map.insert("Milly", "Gavin");
    map.insert("illyM", "Milly");

    println!("map[Milly]: {}",map.get("Milly").unwrap());
    println!("map[illyM]: {}",map.get("illyM").unwrap());
    // map.insert(10, 100);
    // map.insert(20, 200);

    // println!("map[Milly]: {}",map.get(10).unwrap());
    // println!("map[illyM]: {}",map.get(20).unwrap());
}
