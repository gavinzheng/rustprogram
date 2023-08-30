static mut meep: u32 = 4;  	// 可变全局变量
static FUUP: u8 = 9; 		// 不可变全局变量
fn main() { 
unsafe { 				// 可变全局变量相关代码必须包含在unsafe的代码块里
println!("Meep is {}", meep); 
meep = 42; 
println!("Meep is now {}", meep); 
}
println!("Fuup is {}", FUUP); 
} 