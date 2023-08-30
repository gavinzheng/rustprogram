fn main() { 
  let file_name = "game.rs"; 
  // match file_name.find('.') { 
  //   None => println!("找不到文件扩展名."), 
  //   Some(i) => println!("文件扩展名: {}", &file_name[i+1..]), 
  // } 

  // if extension(file_name) == Some(ext){
    println!("文件扩展名: {}", extension(file_name).unwrap());
  // }
}

// 使用map取代match 
fn extension(file_name: &str) -> Option<&str> { 
  file_name.find('.').map(|i| &file_name[i+1..])	 // 如果文件名中有.，
} 