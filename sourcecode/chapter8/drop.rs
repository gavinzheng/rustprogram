struct student { 
  name: String,
  no: i32 
} 
impl Drop for student { 
  fn drop(&mut self) { 
    println!("{} 离开作用域", self.name) 
  } 
} 
fn main() { 
  let zheng= student { name: "嘉文".into(), no: 1000 }; 
  let mi = student { name: "米莉".into(), no: 1001 }; 
} 