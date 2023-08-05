fn joinuse() { 
  let strOrig = Box::new(String::new()); 
  let pointer = &*strOrig ; 
  println!("{} {}", pointer , strOrig ); 
} 
fn separate() { 
  let strOrig = Box::new(String::new()); 
  let temp = *strOrig ; 
  let pointer = &temp ; 
  println!("{} {}", pointer , strOrig ); 
} 
fn main() { 
  joinuse(); 
  separate(); 
}