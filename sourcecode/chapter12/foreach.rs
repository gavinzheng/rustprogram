

fn main(){
  ["alice", "bob", "milly"].iter() 
    .zip(["sugar", "fruit", "yogurt"].iter()) 
    .zip(2..5) 
    .rev() 
    .map(|((item, kind), quantity)| { 		// 闭包
    format!("{} {} {}", quantity, kind, item) 
    }).for_each(|gift| { 				// 闭包
    println!("You have received: {}", gift); 
    });
}