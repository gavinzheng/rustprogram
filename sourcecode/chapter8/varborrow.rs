fn main() { 
    let s=String::from("R in 8 Hours"); 
    let n=cal(&s); // 引用
    println!("字符串内容是: {}",s); 
    println!("字符串长度是: {}",n); 
} 
fn cal(s:&String) -> usize { // 引用
    s.len() // 返回字符串长度 
} 