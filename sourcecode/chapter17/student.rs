struct Student { 
  name: &'static str, 
  score: u32, 
  level: u32 
} 
struct StudentBuilder { 
  name: &'static str, 
  score: u32, 
  level: u32 
} 
impl StudentBuilder { 
  fn new() -> Self { 
    StudentBuilder { name: "Walker", score: 100, level: 10 } 
  } 
  fn name(&mut self, n: &'static str) -> &mut StudentBuilder { 
    self.name = n; 
    self 
  } 
  fn score(&mut self, h: u32) -> &mut StudentBuilder { 
    self.score = h; 
    self 
  } 
  fn level(&mut self, d: u32) -> &mut StudentBuilder { 
    self.level = d; 
    self 
  } 
  fn finish(&self) -> Student { 
    Student { name: self.name, score: self.score, level: self.level } 
  } 
}
fn main() { 
  let al1 = StudentBuilder::new() 			// 1)new构造器
              .name("Entry") 			                // 2)设置一些属性
              .score(60) 
              .level(1) 
              .finish(); 				                    // 3)生成实例
  println!("name: {}", al1.name); 
  println!("score: {}", al1.score); 
} 