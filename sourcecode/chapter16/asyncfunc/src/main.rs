use futures::executor::block_on;
async fn hello_welcome() {
    hello_milly().await;  // 由于使用了await，下面的println！语句必须等待hello_milly异步返回才能执行
    println!("hello, welcome!");
}
async fn hello_milly() {
    println!("hello, Milly!");
}
fn main() {
    let future = hello_welcome();
    block_on(future);
}