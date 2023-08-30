#[macro_use]
extern crate log;
extern crate env_logger;
extern crate app_dirs;
extern crate libc;

// extern crate bitcrypto;
extern crate keys;
extern crate primitives;

use keys::{KeyPair, generator::{self, Generator}};

const SECRET_BOOK: &'static str = "KxFC1jmwwCoACiCAWZ3eXa96mBM6tb3TYzGmf6YwgdGWZgawvrtJ";
const ADDRESS_BOOK: &'static str = "1J7mdg5rbQyUHENYdx39WVWK7fsLpEoXZy";

fn main() {
	::std::env::set_var("RUST_BACKTRACE", "1");
    
	let kp = keys::generator::Random::new(keys::Network::Mainnet).generate().unwrap();

	// let kp = KeyPair::from_private(SECRET_BOOK.into()).unwrap();
	println!("keypair Private: {} ",kp);
	println!("Public Key: {} ",kp.public());

	let address = kp.address();
  println!("address: {} ",address)
}
