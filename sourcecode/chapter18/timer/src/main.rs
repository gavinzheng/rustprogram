#[macro_use] 
//extern crate lazy_static;
extern crate pretty_env_logger ; // RUST_LOG = error
#[macro_use] 
extern crate log;
//extern crate futures;
// extern crate timer;
// extern crate chrono;
// extern crate tokio_timer;

use log4rs;

use std::env;
use warp::{Filter, Rejection};
use tokio::time::{interval, Duration};

pub async fn transchecktimer()-> web3::Result<()>{
    println!("In transactiontimer ...");
    Ok(())
}
async fn manage_timer(){
    let mut interval = interval(Duration::from_millis(10000)); // 设定定时间隔为10秒
    loop {  // 无限循环
        interval.tick().await;  // 等待10秒
        match transchecktimer().await{	// 执行定时器处理函数/handler
            Ok(()) =>(),
            Err(e) =>{
                debug!("transactiontimer error={}",e);
                continue;  // 即使错误，也不退出，进入下一次循环
            }
        }
        }
    }

#[tokio::main]
async fn main() -> web3::Result<()> {
    // 设置环境变量
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    dotenv::dotenv().ok(); // 读取环境配置文件
    // 读入日志配置文件并启动日志
    log4rs::init_file("log_config_all.yaml", Default::default()).unwrap(); 

        // 生成一个线程(Thread)
    tokio::spawn(manage_timer());

    let root = warp::path::end().map(|| "Welcome to Rust Programming Timer Example");
    let routes = root
                    // .or(zkpcc_deposit_route)
                    // .or(zkpcc_generateproof_route)
                    // .or(zkpcc_spendccnote_route)
                    // .or(zkpcc_balance_route)
                    // .or(zkpcc_loadnote_route)
                    .with(warp::cors().allow_any_origin());
                    // .recover(errors::handle_rejection);

    warp::serve(routes).run(([0,0,0,0], 8554)).await;
    
    Ok(())
}