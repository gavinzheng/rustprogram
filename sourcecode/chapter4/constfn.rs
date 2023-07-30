const fn salt(a: u32) -> u32 { 
    0xDEADBEEF ^ a 
  } 
  const CHECKSUM: u32 = salt(56); 
  fn main() { 
    println!("{}", CHECKSUM); 
  }