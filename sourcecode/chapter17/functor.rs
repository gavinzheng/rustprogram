fn main() { 
  let sentences = vec!["this is a test","text have many sentences"]; 
  let words:Vec<&str> = sentences.iter().flat_map(|&x| x.split(" ")).collect(); 
  println!("{:?}", sentences); 
  println!("{:?}", words); 
  } 