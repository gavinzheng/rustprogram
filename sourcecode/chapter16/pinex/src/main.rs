#![feature(async_closure)]
fn main() {
    async fn func1() -> i32 { 56 }

    let func2 = async || -> i32 {
        let t = 10;                  
        let v = t + 10; 
        let rv = &v; 
        let b = func1().await;
//        let rv = &v;   
        *rv + b
    };

    let fut = func2();
    println!("future size: {}", std::mem::size_of_val(&fut));
}