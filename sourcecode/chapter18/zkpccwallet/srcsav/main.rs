#[macro_use] extern crate lazy_static;
extern crate pretty_env_logger ; // RUST_LOG = error
#[macro_use] 
extern crate log;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
// extern crate timer;
// extern crate chrono;
// extern crate tokio_timer;

// use ethane::{Connection,Http};
// use ethane::types::*;
// use ethane::rpc;

use log4rs;

use std::env;
use std::str::FromStr;
//use std::sync::Mutex;
use tokio::sync::Mutex;
use std::{collections::HashMap, /*convert::Infallible,*/ sync::Arc};
// use std::time::{Instant};
// use tokio::task;
//use tokio::prelude::*;
// use tokio::time::Interval;
// use tokio::time;
//use tokio_timer::*;

// use tokio::sync::Mutex;
use warp::{Filter, Rejection};
use web3::types::{H256, U256, BlockNumber, Address};
// use futures::{stream, Stream, TryStreamExt};
// use futures_timer::Delay;
// use timer::Timer;
// use chrono::Duration;
// use std::thread;

// use std::path::Path;
// use std::fs;
// use std::io::Read;
// use std::io::Write;
// use std::fs::File;

// use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};
//use hex_literal::*;

use web3::{
    // ethabi::ethereum_types::U256,
    types::{FilterBuilder},
    // futures::StreamExt,
    // confirm,
};
// use web3::contract::{Contract,Options};
// use web3::contract::tokens::Tokenize;

use crate::api::*; // {DepositNoteRequest, SpendNoteRequest, CCTRANSItem, CCTRANSSTATUS, SpendProofJSON};
// use crate::config::{};
// use crate::ethnoteproxy::EthNoteProxy;
// use crate::transactiontimer::TransactionTimer;
use crate::notejson::{writejson, /*readjson, readnotejson*/};
// use crate::zokcommand::{zokcmd_computewitness, zokcmd_generateproof};
// use crate::filter::*;
// use crate::config::*;
// use crate::utils::*;

mod api;
mod handler;
// mod ethnoteproxy;
// mod bscnoteproxy;
mod errors;
mod config;
mod notejson;
mod zokcommand;
mod utils;

// pub CC_TRANS_STATUS_MAPPING: Lazy<HashMap<H256, CCTRANSItem>>= Lazy::new(||{
//     let mut cctransmap: HashMap<H256, CCTRANSItem> = HashMap::new();
//     cctransmap
// });

// pub static CC_TRANS_List : Lazy<Vec<H256>> = Lazy::new(||{
//     let mut cctranslist : Vec<H256> = Vec::new();
//     cctranslist
// });

lazy_static! {
    // Arc<Mutex<HashMap<String, Character>>>
    pub static ref CC_TRANS_NOTES: Arc<Mutex<HashMap<u64, CCTRANSItem>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref CC_TRANS_STATUS_MAPPING: Arc<Mutex<HashMap<H256, CCTRANSItem>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref CC_TRANS_LIST : Arc<Mutex<Vec<H256>>> = Arc::new(Mutex::new(Vec::new()));
    //pub static ref USERMAPPING: HashMap<u64, UserProfile>;
}

// use crate::bscnoteproxy::BSCNoteProxy;

type Result<T> = std::result::Result<T, Rejection>;

// use serde::de::DeserializeOwned;
use tokio::time::{interval, Duration};

// fn with_json_body<T: DeserializeOwned + Send>(
// ) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
//     // When accepting a body, we want a JSON body
//     // (and to reject huge payloads)...
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }

// fn with_proxy(proxy: EthNoteProxy) -> impl Filter<Extract = (EthNoteProxy,), Error = Infallible> + Clone {
//     warp::any().map(move || proxy.clone())
// }
pub async fn transchecktimer()-> web3::Result<()>{
    println!("In transactiontimer ...");

    let mut cctrans_to_remove : Vec<H256> = Vec::new();
    
    let mut txmap = CC_TRANS_STATUS_MAPPING.lock().await;
    let vec: Vec<(H256, CCTRANSItem)> = // txmap.into_iter().collect();
            txmap.iter()
            .map(|(id, item) | (id.clone(), item.clone()))
            // .copied()
            .collect();
    
    for i in 0..vec.len() {
        debug!("Transaction {:?}", vec[i].0);
        
        // let curitem = vec[i].clone();

        let mut txitem = txmap.get_mut(&vec[i].0).unwrap();

        if CCTRANSSTATUS::Included != txitem.status && CCTRANSSTATUS::Sent != txitem.status { continue;} 

        let chainid = if CCTRANSSTATUS::Sent == txitem.status{
            vec[i].1.request.fromchainid
        }else{
            vec[i].1.request.tochainid
        };

        // Connection    
        let httpobj = if chainid == std::env::var("ETH_CHAIN_ID").unwrap().parse::<u64>().unwrap() { // 2
            web3::transports::Http::new(&std::env::var("INFURA_ROPSTEN").unwrap())//.unwrap()
        }else if chainid == std::env::var("BSC_CHAIN_ID").unwrap().parse::<u64>().unwrap() { // 79 ###
            web3::transports::Http::new(&std::env::var("BSC_ENDPOINT").unwrap())//.unwrap()
        }else {
            debug!("Invalid Chain ID in TransactionTimer");
            continue
        };
        if httpobj.is_err(){
            continue;
        }
        let http = httpobj.unwrap();

        let web3 = web3::Web3::new(http);
        let mut curblockno = 0 ; 
        let mut chainswitch = false; 
        
        // start block
        let mut startblockno = txitem.blockno ;
        if txitem.blockno ==0 {
            startblockno = if chainid == 2 {
                std::env::var("ETH_START_BLOCKN0").unwrap().parse::<u64>().unwrap()  // "11670277" Todo: read from config csv
            }else  {
                // txitem.request.tochainid == 79
                std::env::var("BSC_START_BLOCKN0").unwrap().parse::<u64>().unwrap()   // "11670277" Todo: read from config csv
            }
        }else {
            let curblocknoobj = web3.eth().block_number().await?;
            curblockno = curblocknoobj.as_u64();
            if txitem.blockno > curblockno{
                startblockno = curblockno;
            }
        }

        let toblockno = startblockno + 2000 ;
        debug!("Start Block = {} => To block ={}, latest block ={}", startblockno, toblockno,curblockno);

        // Filter Setting
        let filter = FilterBuilder::default()
            .address(vec![crate::config::noteproxylookup(chainid)])
            .from_block(BlockNumber::Number(web3::types::U64::from_dec_str(&format!("{}",startblockno).to_string()[..]).unwrap()))
            .to_block(BlockNumber::Number(web3::types::U64::from_dec_str(&format!("{}",toblockno).to_string()[..]).unwrap()))
            .topics(
                Some(vec![H256::from_str(&std::env::var("ZKPOUTPUT_EVENT").unwrap()).unwrap(),
                H256::from_str(&std::env::var("ZKPINPUT_EVENT").unwrap()).unwrap()]),
                None,
                None,
                None,
            )
            .build();

        let loglist = web3.eth().logs(filter).await?;//.unwrap();
        debug!("loglist={}", loglist.len());

        for j in 0..loglist.len(){
            let datamerkle = loglist[j].data.0.clone();

            match txitem.status {
                CCTRANSSTATUS::Sent =>{
                    // if the log is unrelated to current txitem then skip
                    if loglist[j].transaction_hash.unwrap() != txitem.fromtx { 
                        continue;  
                    }

                    if loglist[j].topics.contains(&H256::from_str(&std::env::var("ZKPOUTPUT_EVENT").unwrap()).unwrap())
                    {
                        // let tochainstartblockno = if txitem.request.tochainid == 2 {
                        //     std::env::var("ETH_START_BLOCKN0").unwrap().parse::<u64>().unwrap()  // "11670277" Todo: read from config csv
                        // }else  {
                        //     // txitem.request.tochainid == 79
                        //     std::env::var("BSC_START_BLOCKN0").unwrap().parse::<u64>().unwrap()   // "11670277" Todo: read from config csv
                        // };
                        let httpalt = if txitem.request.tochainid == std::env::var("ETH_CHAIN_ID").unwrap().parse::<u64>().unwrap() { // 2
                            web3::transports::Http::new(&std::env::var("INFURA_ROPSTEN").unwrap()).unwrap()
                        }else if txitem.request.tochainid == std::env::var("BSC_CHAIN_ID").unwrap().parse::<u64>().unwrap() { // 79
                            web3::transports::Http::new(&std::env::var("BSC_ENDPOINT").unwrap()).unwrap()
                        }else {
                            debug!("Invalid Chain ID in processing CCTRANSSTATUS::Sent TransactionTimer");
                            continue
                        };
                        let web3alt = web3::Web3::new(httpalt);
                        let tochainstartblockno = web3.eth().block_number().await?.as_u64();

                        // ZKPOutputEvent
                        // event ZKPOutputEvent(bytes32 noteid, bytes toAssetHash, address fromAssetHash, uint64 toChainId, uint256 amount, uint256 pk1, uint256 pk2, string encryptedNote, address fromaddress/*, bytes32[4] arr, bool[2] directionSelector*/);
                        // bytes32 noteid, bytes toAssetHash, address fromAssetHash, uint64 toChainId, uint256 amount, uint256 pk1, 
                        // uint256 pk2, string encryptedNote, address fromaddress, bytes32[4] arr, bool[2] directionSelector
                        txitem.noteid = U256::from_big_endian(&datamerkle[0..32]);
                        txitem.fromaddress = Address::from_slice(&datamerkle[268..288]);
                        txitem.status = CCTRANSSTATUS::Included;
                        txitem.blockno= tochainstartblockno; // blocknumber on the other chain  // loglist[j].block_number.unwrap().as_u64();
                        curblockno = tochainstartblockno;
                        let to_asset_hash= U256::from_big_endian(&datamerkle[320..340]);

                        debug!("noteid={0:x}, \n fromAddress=={1:x}, \n toAssethash={2:x},\n amount={3:?}",
                                txitem.noteid , txitem.noteid , to_asset_hash, txitem.request.amount);   

                         // writejson(serde_json::to_string_pretty(&note).unwrap().to_string().as_bytes(), noteid.to_string());
                        match writejson(serde_json::to_string_pretty(&txitem).unwrap().to_string().as_bytes(), txitem.noteid.to_string()){
                            Ok(()) => (),
                            Err(err) =>{
                                println!("Can not write note in CCTRANSSTATUS::Sent: {} Error={}", txitem.noteid, err);
                                info!("Can not write note in CCTRANSSTATUS::Sent: {} ", txitem.noteid);
                            }
                        };

                        chainswitch = true;    
                    }    
                } 
                CCTRANSSTATUS::Included =>{
                    // ZKPIput Event
                    if loglist[j].topics.contains(&H256::from_str(&std::env::var("ZKPINPUT_EVENT").unwrap()).unwrap())
                    {
                        // event ZKPInputEvent(bytes32 noteid, bytes32 reqhash, bytes toAssetHash, uint256 amount,bytes32[4] arr, bool[2] directionSelector);
                        let noteid= U256::from_big_endian(&datamerkle[0..32]);
                        if noteid != txitem.noteid {
                            continue;
                        }    
                        // debug!("noteid={0:x}, \n reqhash=={1:x}, \n toAssethash={2:x},\n amount={3:?}",
                        //     noteid, reqhash, to_Asset_hash, amount);   
                        // match readnotejson(noteid.to_string()) {
                        //     Err(err)   => { 
                        //         println!("Can not read note: {} Error={}", noteid, err);
                        //         info!("Can not read note: {} ", noteid);
                        //     }
                        //     Ok(note)   => {
                                txitem.notereq= U256::from_big_endian(&datamerkle[32..64]);
                                //txitem.amount= U256::from_big_endian(&datamerkle[64..96]);
                                txitem.pathdigest0= U256::from_big_endian(&datamerkle[128..160]);
                                txitem.pathdigest1= U256::from_big_endian(&datamerkle[160..192]);
                                txitem.rootdigest= U256::from_big_endian(&datamerkle[192..224]);
                                txitem.leaf = U256::from_big_endian(&datamerkle[224..256]);
                                let direction0 = U256::from_big_endian(&datamerkle[256..288]); 
                                let direction1 =  U256::from_big_endian(&datamerkle[288..320]);
                                txitem.direction= [direction0, direction1];
                                txitem.blockno=loglist[j].block_number.unwrap().as_u64();
                                let to_asset_hash= U256::from_big_endian(&datamerkle[352..384]);

                                txitem.status = CCTRANSSTATUS::Transferred;
                                // txitem.noteid = noteid;
                                // txitem.notereq = reqhash;
                                // txitem.toAssethash = toAssethash;
                                // txitem.blockno = web3.eth().block_number().await.unwrap().as_u64() ;

                                // writejson(serde_json::to_string_pretty(&note).unwrap().to_string().as_bytes(), noteid.to_string());
                                match writejson(serde_json::to_string_pretty(&txitem).unwrap().to_string().as_bytes(), noteid.to_string()){
                                    Ok(()) => (),
                                    Err(err) =>{
                                        println!("Can not write note in OK arm: {} Error={}", noteid, err);
                                        info!("Can not write note: {} ", noteid);
                                    }
                                }; 

                                // #####################################################
                                // copy the successful CC transaction for Query???
                                let mut ccnotesmap = CC_TRANS_NOTES.lock().await;
                                ccnotesmap.insert(79, txitem.clone());
                                
                                // remove from CC transaction monitoring map
                                //CC_TRANS_STATUS_MAPPING.remove(txitem.fromtx);
                                // CC_TRANS_LIST.remove(list[i]);
                                // txmap.remove(&note.fromtx);
                                cctrans_to_remove.push(vec[i].0.clone());    
                                // txmap.remove(tx);    
                        //     }
                        // };
                            
                    }

                } 
                _ => debug!("Abnomal State ...")
            }
        }

        if chainswitch == false {
            let latestblockno = curblockno; // web3.eth().block_number().await.unwrap().as_u64();
            txitem.blockno = if toblockno > latestblockno {
                latestblockno
            }else{
                toblockno
            }
        }
        debug!("------------------------ txitem.blockno ={}", txitem.blockno);

    }

    // Remove completed transactions from monitor mapping
    if cctrans_to_remove.len() > 0 {
        // let mut txmap = CC_TRANS_STATUS_MAPPING.lock().await;
        if txmap.is_empty() { debug!("txmap is null");}
        
        // Do not need to empty CCTranstoRemove
        for k in 0..cctrans_to_remove.len(){
            txmap.remove(&cctrans_to_remove[k]);
        }
    }
    Ok(())
}

async fn manage_timer(){
    let mut interval = interval(Duration::from_millis(10000));
    // transchecktimer().await;
    loop {
        interval.tick().await;
        match transchecktimer().await{
            Ok(()) =>(),
            Err(e) =>{
                debug!("transactiontimer error={}",e);
                continue;
            }
        }
    }
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    // Environment
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    dotenv::dotenv().ok();

    // println!("usermapping length={}", USERMAPPING.is_empty());
    
    // let obj = read_conf("project.json").expect("Could not load project.json");
    // let data = fs::read_to_string("/etc/hosts").expect("Unable to read file");

    // log
    // pretty_env_logger::init();
    log4rs::init_file("log_config_all.yaml", Default::default()).unwrap();

    // Test data simulator
    // tokio::spawn(async move{
    // //     let tx = H256::from_str("0xd0a27991961faf7b13c70c0fd2be82dce0c64cc13a9905506ab68a531db70fc2").unwrap() ;
    // //     CC_TRANS_LIST.lock().await.push(tx);
    
    //         let mut transmap = CC_TRANS_STATUS_MAPPING.lock().await;
    //         let noteid = U256::from_str("0xfd128843e0356939ea32e595f5595dec953c1e2440c5f71cf444f76c60adb2f5").unwrap();
    //         match readnotejson(noteid.to_string()) {
    //             Err(err)   => { 
    //                 println!("Can not read note: {} Error={}", noteid, err);
    //                 info!("Can not read note: {} ", noteid);
    //             }
    //             Ok(note)   => {
    //                 transmap.insert(
    //                     note.fromtx, note
    //                 );
    //             }
    //         };

    //     let item = CCTRANSItem{
    //         request : DepositNoteRequest{
    //             pk1: U256::from_dec_str("17559621506966228669946200908540191909304602050620774623186985148217036891086").unwrap(),
    //             pk2: U256::from_dec_str("3499692685359225014642568034871893823673913644950320594232804843053171036306").unwrap(),
    //             amount: U256::from_dec_str("200000000").unwrap(),
    //             tochainid : 79,
    //             fromassethash : "473850cB9b3Ee1225304834DE679521d7f21d52f".to_string(),
    //             encryptednote : "test001".to_string(),     // Detail 
    //             fromchainid : 2,            // Deposit from Chain ID
    //         },
    //         noteid: U256::from_str("0x73453476b05f2021657e4d28d08ea4cc8f92c3abb972517a24cc746e73867153").unwrap() ,
    //         notereq: U256::from_dec_str("0").unwrap(),
    //         rootdigest: U256::from_str_radix(&"0xa3f84c28e61a7c9da09f4f2e46616bf073c4845f65404e4e64050c4301cf3f7b", 16).unwrap(),
    //         leaf : U256::from_str_radix(&"0xc38af55eab1e6956ab1f7e3e1c2250ff1570fc89fd98a5646dfe8d3a480e1696", 16).unwrap(),
    //         pathdigest0: U256::from_str_radix(&"0x0000000000000000000000000000000000000000000000000000000000000000",16).unwrap(),
    //         pathdigest1: U256::from_str_radix(&"0xf5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b",16).unwrap(),
    //         direction: [U256::from_dec_str("0").unwrap(),U256::from_dec_str("0").unwrap(),],
    //         privkey: U256::from_dec_str("37232355431795428394424168963998030788617641514587324629047050974144141575298").unwrap(),
    //         status: CCTRANSSTATUS::Included,
    //         fromtx: tx,
    //         blockno: 14478756,
    //         fromaddress: Address::from_str("0").unwrap(),
    //         // toAssethash:H256::from_str("0").unwrap(),
    //     };
        
    //     // writejson(serde_json::to_string_pretty(&item).unwrap().to_string().as_bytes(), "52138252125203178713318795450414024069006896720816266979072757700350127272275".to_string());
    //     writejson(serde_json::to_string_pretty(&item).unwrap().to_string().as_bytes(), "test002".to_string());

    //     transmap.insert(
    //         tx, item
    //     );
    // }).await;

    // for (k, v) in &filters {
    //     match k {
    //         2 => continue, 
    //         79=>{
    //             let mut conn = Connection::new(Http::new("https://data-seed-prebsc-1-s1.binance.org:8545/", None));
    //             // eth_newFilter
    //             let filter = ethane::types::Filter{
    //                 from_block: Some(BlockParameter::Custom(v.startblock)),
    //                 to_block: Some(BlockParameter::Latest),
    //                 address: Some(ValueOrVec::Value(v.contract)),
    //                 topics: Some(vec![Some(ValueOrVec::Vec(
    //                     v.topics.iter().cloned().collect()
    //                 ))]),
    //             };
    //             let filterid = conn.call(rpc::eth_new_filter(filter)).unwrap();
    //             println!("{}", filterid);
    //         }
    //         _ => debug!("index out of range"),                
    //     }
    // }

    tokio::spawn(manage_timer());

    let root = warp::path::end().map(|| "Welcome to ZKP Cross Chain Wallet");

    let zkpcc_deposit_route = warp::path("depositccnoteout")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::deposit_ccnote_out);

    let zkpcc_generateproof_route = warp::path("generateproof")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::generateproof);

    let zkpcc_spendccnote_route = warp::path("spendccnote")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::spend_ccnote);

    let zkpcc_balance_route = warp::path("balance")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::balance);

    let zkpcc_loadnote_route = warp::path("loadnote")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::load_note);

    let routes = root
        .or(zkpcc_deposit_route)
        .or(zkpcc_generateproof_route)
        .or(zkpcc_spendccnote_route)
        .or(zkpcc_balance_route)
        .or(zkpcc_loadnote_route)
        .with(warp::cors().allow_any_origin())
        .recover(errors::handle_rejection);

    warp::serve(routes).run(([0,0,0,0], 8554)).await;

    Ok(())
}