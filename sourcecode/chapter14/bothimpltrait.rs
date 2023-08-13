use std::fmt::Display; 
fn surround_with_braces(val: impl Display) -> impl Display { 
format!("{{{}}}", val) 	// 必须用额外的{}来转义，这样format！宏才能正确地进行字符串展开
} 
fn main() { 
println!("{}", surround_with_braces("Hello")); 
} 