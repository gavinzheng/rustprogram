use chrono::prelude::*;
use std::env;
use web3::types::{BlockId, BlockNumber};
#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let http = web3::transports::Http::new(&std::env::var("INFURA_ROPSTEN").unwrap()).unwrap()
  let web3s = web3::Web3::new(http);
  let latest_block = web3s
          .eth()
          .block(BlockId::Number(BlockNumber::Latest)) // 指定最新块块号
          .await
          .unwrap()
          .unwrap();
  println!("block number {}, number of transactions: {}, difficulty {}",
    latest_block.number.unwrap(),				// 最新块块号
    &latest_block.transactions.len(),			// 最新块中交易个数
    &latest_block.total_difficulty.unwrap()	// 最新块中挖矿难度
  );
}