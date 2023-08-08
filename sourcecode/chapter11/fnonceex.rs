fn main() { 
  let mut a = Box::new(23); 
  let call_me = || { 
  let c = a; 
  }; 
  call_me(); 
  call_me(); 
} 