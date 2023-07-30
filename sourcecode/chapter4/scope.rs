fn main() { 
	let outer = 42; 
	{ // start of code block 
		let inner = 3.14; 
		println!("block 变量: {}", inner); 
		let outer = 99; // 遮蔽了外部的outer变量 
		println!("block 变量 outer: {}", outer); 
	} // end of code block 
	println!("outer 变量: {}", outer); 
}