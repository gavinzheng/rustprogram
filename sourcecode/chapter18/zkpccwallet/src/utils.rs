
// Ethereum/Ropsten
// use web3::contract::{Contract,Options};
// use web3::transports::Http;

// pub fn get_proxy_contract (chainid : u32) -> Contract<Http> {
//     let infurakey =
//         std::env::var("INFURA_ROPSTEN").expect("Please set the INFURA_PROJECT env variable");

//     let http = web3::transports::Http::new(&infurakey).unwrap();
//     let web3 = web3::Web3::new(http);

//     let proxycontract =
//         Contract::from_json(web3.eth(), crate::config::noteproxylookup(chainid), include_bytes!("../abi/erc20ZKPNoteproxyMerkle.json")).unwrap();

//     proxycontract
// }

// pub fn ethanetoWeb3address(ethaneaddr: ethane::types::Address) -> web3::types::Address{
//     let ret = web3::types::Address::from_slice(&ethaneaddr.into_bytes());
//     ret
// }
// pub fn web3toethaneH256(web3h256: web3::types::H256 ) -> ethane::types::H256{
//     let ret = ethane::types::H256::try_from(web3h256.as_bytes()).unwrap();
//     ret
// }
use secp256k1::SecretKey;
use web3::types::U256;

// pub fn u256_to_secretkey(key: U256)-> SecretKey{
//     // let private_key = SecretKey::from_str(&env::var("ETH_PRIVATE_KEY").unwrap()).unwrap();
//     let mut bytespriv: [u8;32]=[0;32];
//     //let privk = U256::from_str("0e4937aeeeaef0a1d33ff35bea2e4a4af99ce311651a711af038ea1a5629061f").unwrap();
//     // let strpriv = key.to_big_endian(&mut bytespriv);
//     //let strpriv = privk.to_big_endian(&mut bytespriv);
//     // debug!("priv key1={}", strpriv);

//     // let strpriv = crate::config::userlookup(req.userid);
//     // let strpriv = crate::config::USERMAPPING
//     //                 .get(&req.userid)
//     //                 .unwrap()
//     //                 .clone(); 
//     // strpriv.sk.to_big_endian(& mut bytespriv);
//     //let strpriv = privk.to_big_endian(&mut bytespriv);
//     // debug!("priv key1={}", strpriv);

//     // let private_key = SecretKey::from_str(&privk.to_string()[..]).unwrap();
//     //let private_key = SecretKey::from_slice(&bytespriv).unwrap();

//     // let private_key = SecretKey::from_str(&privk.to_string()[..]).unwrap();
//     SecretKey::from_slice(&bytespriv).unwrap()
// }
pub fn u256_to_secretkey(key: U256)-> SecretKey{
    let mut bytespriv: [u8;32]=[0;32];
    key.to_big_endian(& mut bytespriv);

    SecretKey::from_slice(&bytespriv).unwrap()
}
// pub fn userid_bytes_to_secretkey(userid : u64)-> SecretKey{
//     // let private_key = SecretKey::from_str(&env::var("ETH_PRIVATE_KEY").unwrap()).unwrap();
//     let mut bytespriv: [u8;32]=[0;32];
//     crate::config::userlookup(userid).sk.to_big_endian(& mut bytespriv);
//     //let strpriv = privk.to_big_endian(&mut bytespriv);
//     // debug!("priv key1={}", strpriv);

//     let private_key = SecretKey::from_slice(&bytespriv).unwrap();
//     private_key
// }