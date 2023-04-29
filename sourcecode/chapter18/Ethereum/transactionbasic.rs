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

  for transaction_hash in latest_block.transactions { // 遍历最新块中所有交易
    let tx = match web3s
              .eth()
              .transaction(TransactionId::Hash(transaction_hash))	// 获取交易哈希，并通过哈希获取交易对象
              .await
      {
        Ok(Some(tx)) => tx,	// 成功则交易对象在tx中
        _ => {				// 失败，则打印错误信息，略过继续
            println!("An error occurred");
            continue;
        }
      };

    // 解封tx.from值，如果失败，则设置H160类型的0
    let from_addr = tx.from.unwrap_or(H160::zero());	// 发起交易地址
    let to_addr = tx.to.unwrap_or(H160::zero());	// 交易对象地址
    println!("[{}] from {}, to {}, value {}, gas {}, gas price {}",
      tx.transaction_index.unwrap_or(U64::from(0)),
      w3h::to_string(&from_addr),
      w3h::to_string(&to_addr),
      tx.value,			// 交易ETH
      tx.gas,			// 交易燃料
      tx.gas_price,		// 交易燃料价格
    );
  }
}