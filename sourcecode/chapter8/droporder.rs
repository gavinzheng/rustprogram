struct EX(String); 
impl Drop for EX { 
  fn drop(&mut self) { 
    print!("{}", self.0); 
  } 
} 

fn main() {
  let a = EX("信链".to_string()); 
  let b; 
  let c = EX("武汉".to_string()); 
  b = EX("北科".to_string()); 
}