use std::time::{Duration, Instant}; 
fn main() { 
 let mut count = 0; 							// 计数器为0
 let time_limit = Duration::new(1,0); 			// 设置1秒时长
 let start = Instant::now(); 					// 获取开始时刻
 
 while (Instant::now() - start) < time_limit { 	// 如果时间间隔小于1秒，则技术器加1	
    count += 1; 
} 
println!("{}", count); 
}