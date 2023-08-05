struct EX(String); 
impl Drop for EX { 
  fn drop(&mut self) { 
    print!("{}", self.0); 
  } 
} 

fn main() {
  let _a = EX("信链".to_string()); 
  let _b; 
  let _c = EX("武汉".to_string()); 
  _b = EX("北科".to_string()); 
}