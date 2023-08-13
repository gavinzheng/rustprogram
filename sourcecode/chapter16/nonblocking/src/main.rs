use tokio::task;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {    
    let task1 = task::spawn(async {        
        println!("Task 1 started");        // 模拟异步任务         
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;        
        println!("Task 1 finished");    
    });    
    let task2 = task::spawn(async {        
        println!("Task 2 started");        // 模拟异步任务        
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;        
        println!("Task 2 finished");    
    });    
  
    let _ = tokio::join!(task1, task2);	// 等待两个任务完成  
}