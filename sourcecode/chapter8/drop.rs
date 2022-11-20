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
  let zhang= student { name: "张三".into(), no: 1000 }; 
  let li = student { name: "李四".into(), no: 1001 }; 
} 