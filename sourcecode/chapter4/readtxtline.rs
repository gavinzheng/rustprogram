use std::fs::File; 
use std::io::{BufRead, BufReader}; 
fn main() { 
let filename = "foo.txt"; 
// 打开文件句柄(默认打开成功)
let file = File::open(filename).unwrap(); 
let reader = BufReader::new(file); 
// 使用std::io::BufRead中的lines()的迭代器按行读取文件内容 
for (index, line) in reader.lines().enumerate() { 
let line = line.unwrap(); // 忽略错误
// 打印行内容和行编号
println!("{}. {}", index + 1, line); 
} 
} 