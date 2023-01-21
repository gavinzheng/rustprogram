struct X(char); 
impl Drop for X { 
  fn drop(&mut self) { 
    print!("{}", self.0); 
  } 
} 

fn main() {
  let _a = X('a'); 
  let _b; 
  let _c = X('c'); 
  _b = X('b'); 
}