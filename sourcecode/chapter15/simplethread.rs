use std::thread; 
fn main() { 
// 创建一个线程 
let child_thread = thread::spawn(move || { 
println!("在新的子线程中..."); 
}); 
child_thread.join().unwrap(); // 等待新建线程执行完成
} 