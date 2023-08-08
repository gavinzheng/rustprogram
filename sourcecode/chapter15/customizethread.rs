use std::thread; 
fn main() { 
// 创建一个线程，线程名称为 thread1, 堆栈大小为4k 
let new_thread_result = thread::Builder::new() 
.name("Worker Thread".to_string()) 	// 设置线程名称
.stack_size(4*1024)					// 设置线程栈大小
.spawn(move || { 					// 闭包作为线程体
println!("I am thread1."); 
}); 
new_thread_result.unwrap().join().unwrap(); 	// 等待新创建的线程执行完成 
}