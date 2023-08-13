struct MyTraitType; 					// 空结构体
unsafe trait UnsafeTrait { 			// 不安全的trait中可以包含不安全和安全的函数	
unsafe fn unsafe_func(&self); 
fn safe_func(&self) { 
println!("不安全的trait中可以包含安全的函数!"); 
} 
} 
trait SafeTrait { 					// 安全的trait可以包含不安全的函数
unsafe fn check_unsafe_call(&self); 
} 
unsafe impl UnsafeTrait for MyTraitType{ 
unsafe fn unsafe_func(&self) { 
println!("不安全的trait中可以包含不安全的函数"); 
} 
} 
impl SafeTrait for MyTraitType{ 
unsafe fn check_unsafe_call(&self) { 
println!("安全的trait中可以包含不安全的函数!"); 
} 
} 
fn main() { 
let owntype = MyTraitType; 
owntype.safe_func(); 
unsafe { 						// 对trait的不安全的函数调用需要封装在unsafe的块中
owntype.check_unsafe_call(); 
} 
}