use std::num::ParseIntError;
fn main() -> Result<(), ParseIntError> {
  let strnumber = "10a";
  let number:i32 = match strnumber.parse() {
        Ok(number) => number,
        Err(_) => panic!("字符串中包含非数字字符") // 异常退出程序
  };
  println!("{}", number); 
  Ok(())
}