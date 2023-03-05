extern crate chrono; 
use chrono::prelude::*; 
use std::io::prelude::*; 
use std::fs::File; 
use std::io; 
fn main() { 
    // 使用本地时间
    // 2023-03-03 16:14:12.086930200 +08:00  
    let local: DateTime<Local> = Local::now(); 
    println!("{}", local); 

    // 格式化输出如下: 
    // Fri, Mar 03 2023 04:13:15 PM
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string(); 
    match log_info("log.txt", &formatted) { 
    Ok(_) => println!("Time is written to file!"), 
    Err(_) => println!("Error: could not create file.") 
    } 
    } 
    fn log_info(filename: &str, string: &str) -> io::Result<()> { 
    let mut f = File::create(filename)?; 
    // try!(f.write_all(string.as_bytes())); 
    f.write_all(string.as_bytes())?; 
    Ok(()) 
} 