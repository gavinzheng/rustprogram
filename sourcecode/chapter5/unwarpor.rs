// fn find(haystack: &str, needle: char) -> Option<usize> {
//   for (offset, c) in haystack.char_indices() {
//     if c == needle {
//       return Some(offset);
//     }
//    }
//   None
//  }
fn extension(file_name: &str) -> Option<&str> { 
    file_name.find('.').map(|i| &file_name[i+1..])	 // 如果文件名中有.，
}
fn main() { 
  assert_eq!(extension("game.exe").unwrap_or("rs"), "exe"); 
  assert_eq!(extension("game").unwrap_or("rs"), "rs"); // 如果没有后缀名(值为None)，则默认返回rs
  } 