use std::env; 
use std::fs::File; 
use std::io::Write; 
fn main() { 
// 创建临时文件
let temp_directory = env::temp_dir(); 
let temp_file = temp_directory.join("file"); 
// 以只写模式打开一个文件。如果文件不存在，则自动创建文件
let mut file = File::create(temp_file).unwrap(); 
// 以&str类型写入文件(忽略错误). 
writeln!(&mut file, "Hello World!").unwrap(); 
// 写入"Byte”字符串
file.write(b"Bytes\n").unwrap(); 
} 