use std::fs::File; 
use std::io::Read; 

fn main() { 
let filename = "foo.txt"; 
// 以只读模式打开文件句柄
match File::open(filename) { 
// 如果打开文件句柄成功 
Ok(mut file) => { 
let mut content = String::new(); 
// 把所有的文件内容读入一个变量(默认该操作成功) 
file.read_to_string(&mut content).unwrap(); 
println!("{}", content); 
// 文件句柄自动关闭
}, 
// 错误处理 
Err(error) => { 
	 println!("Error opening file {}: {}", filename, error); 
}, 
} 
} 