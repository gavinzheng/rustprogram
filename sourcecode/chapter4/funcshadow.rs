fn f() { print!("1"); } 			// 外部函数
fn main() { 
f(); 						// Prints 2	调用外部函数	
{ 
f(); 					// Prints 3 	调用内部函数
fn f() { print!("3"); } 	// 遮蔽外部f函数
} 
f(); 						// Prints 2 调用main块中的f函数 
fn f() { print!("2"); } 
}