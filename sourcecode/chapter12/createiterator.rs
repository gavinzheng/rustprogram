fn main() { 
  let mut names = vec!["Alice", "Bob", "Milly"]; 
  for name in names.iter_mut() { 
    match name { 
      &mut "Milly" => println!("Happy birthday, Milly!"), 
      _ => println!("Hello {}", name), 
    } 
  } 
  println!("{:?}",names);// 在迭代访问后服用names集合
}