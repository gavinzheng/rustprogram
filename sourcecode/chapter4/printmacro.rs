fn main(){ 
	// 打印精确到小数点后2位 
	println!("{:.2}",1.2345 ); 
	println!("================"); 
	// 以2进制，16进制和八进制方式打印
	println!("B: {:b} H: {:x} O: {:o}",10,10,10 ); 
	println!("================"); 
	// Shifts 
	println!("{ten:>ws$}",ten=10, ws=5 ); 
	println!("{ten:>0ws$}",ten=10, ws=5 ); 

	println!("pi is {:e} in floating point notation", PI); // 3.14e0 
} 