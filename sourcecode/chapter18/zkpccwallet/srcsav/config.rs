// #[macro_use] extern crate lazy_static;
// use std::env;
use once_cell::sync::Lazy;
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
// use hex_literal::*;
use std::str::FromStr;

use web3::types::{Address, H256, U256};

pub fn noteproxylookup(chainid: u64) -> Address {
    let addr = NOTEPROXYMAPPING
        .get(&chainid)
        .unwrap()
        .clone();
    println!("addr={}",addr);
    addr
}
pub fn userlookup(id: u64) -> UserProfile {
    let userprofile = USERMAPPING
        .get(&id)
        .unwrap()
        .clone();
    userprofile
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserProfile{
    pub pk1 : U256,
    pub pk2: U256,
    pub sk : U256, 
    pub addr : Address,
}

// Listen on Events: Merkle events, ZKOutputEvent, ZKInputEvent etc
// which is defined in Noteproxy Contract
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct EventSetting{
    pub topics : Vec<H256>,
    pub contract: Address,
    pub filterid : u64, 
    pub startblock: u64,
}
// Listen on Events: Merkle events, ZKOutputEvent, ZKInputEvent etc
// which is defined in Noteproxy Contract
lazy_static! {
    pub static ref NOTEEVENTNMAPPING: Arc<Mutex<HashMap<u64, EventSetting>>> = Arc::new(Mutex::new({
        let m = HashMap::from([
            // Ethereum / Ropsten /Rinkeby
            // MerkleInsertEvent(nextIndex - 1, rtDigest, _leaf,  direction, digest1, digest2 );
            (2, EventSetting{   topics: vec![
                                            H256::from_str(&std::env::var("ZKPOUTPUT_EVENT").unwrap()).unwrap(),
                                            H256::from_str(&std::env::var("ZKPINPUT_EVENT").unwrap()).unwrap(),
                                            ],
                                contract: Address::from_str(&std::env::var("ETH_NOTE_PROXY").unwrap()).unwrap(),
                                filterid: 0,
                                startblock: 11588103,
            }),

            // BSC
            // DeserializeEvent(ccnotereq.toAssetHash,ccnotereq.amount,ccnotereq.pk1,ccnotereq.pk2,ccnotereq.toChainId,ccnotereq.noteid);
            // ZKPInputEvent(ccnotereq.noteid, reqHash, ccnotereq.toAssetHash, ccnotereq.amount);
            (79, EventSetting{  topics: vec![
                                H256::from_str(&std::env::var("ZKPOUTPUT_EVENT").unwrap()).unwrap(),
                                H256::from_str(&std::env::var("ZKPINPUT_EVENT").unwrap()).unwrap(),
                                ],
                                contract:Address::from_str(&std::env::var("BSC_NOTE_PROXY").unwrap()).unwrap(),
                                filterid: 0,
                                startblock: 14478756,
            }),
        ]);
        m
    }));
    pub static ref USERMAPPING: Arc<HashMap<u64, UserProfile>> = Arc::new({HashMap::from([
        // Account2
        (2, UserProfile{   
                pk1: U256::from_dec_str("20135344607339334622863299803442837754032803069560704307911915497149458191874").unwrap(),
                pk2: U256::from_dec_str("2187595016756003212310795244301815584643484035802515093924721825138467143092").unwrap(),
                sk:  U256::from_dec_str("6461744028110636604789666794292708634408021349981947684103482452543804081695").unwrap(),
                addr: Address::from_str("F6478e9dc1Ec91200fAfF28237f259C03369c784").unwrap(),
        }),
        // Account10
        (10, UserProfile{   
                pk1: U256::from_dec_str("12535452613966674258666660089207652538839432851494982167756006372609550828057").unwrap(),
                pk2: U256::from_dec_str("9307290662741456412336367788815074887249314829867085321442656736288754426574").unwrap(),
                sk:  U256::from_str("a63df2bd774d6bd45b36163d296a6a79ea616b40f8573ca347a023d83f2f8f8e").unwrap(),
                addr: Address::from_str("Af2518cB013d9E917ecf11ea1e4F8d16d2E913ec").unwrap(),
        }),
        // Account9
        (9, UserProfile{   
            pk1: U256::from_dec_str("17559621506966228669946200908540191909304602050620774623186985148217036891086").unwrap(),
            pk2: U256::from_dec_str("3499692685359225014642568034871893823673913644950320594232804843053171036306").unwrap(),
            sk:  U256::from_dec_str("37232355431795428394424168963998030788617641514587324629047050974144141575298").unwrap(),
            addr: Address::from_str("22D1aB364210E18576621c4744CeF419cC2Df94A").unwrap(),
        }),
    ]

    )});
}

// lazy_static! {
//     pub static NOTEEVENTNMAPPING: Mutext<HashMap<u32, EventSetting>> = Mutex::new(HashMap::new({
//         // TODO: Read these from a CSV?
//         let noteeventmap: HashMap<u32, EventSetting> = [
//             // Ethereum / Ropsten /Rinkeby
//             // MerkleInsertEvent(nextIndex - 1, rtDigest, _leaf,  direction, digest1, digest2 );
//             // (201, "ff3931ac4000422df2e1c5d545135577b39d592e02282ca882baf31abdb2356b"),   // Merkle
//             // (202, "ff3931ac4000422df2e1c5d545135577b39d592e02282ca882baf31abdb2356b"),   // ZKOutput
//             (2, EventSetting{   topics: vec![H256::try_from("ff3931ac4000422df2e1c5d545135577b39d592e02282ca882baf31abdb2356b").unwrap()],
//                                 contract: Address::try_from("0x88a764DBf06efA77c599ad08FBe362A30b41C47B").unwrap(),
//                                 filterid: 0,
//                                 startblock: U64::try_from_int(11588103_u64).unwrap(),
//             }),

//             // BSC
//             // // DeserializeEvent(ccnotereq.toAssetHash,ccnotereq.amount,ccnotereq.pk1,ccnotereq.pk2,ccnotereq.toChainId,ccnotereq.noteid);
//             // (7900, "1a95c27c7cb22d2091f867137cb4d13c851b219adc23cf1af2e9d355eba66981"),
//             // // ZKPInputEvent(ccnotereq.noteid, reqHash, ccnotereq.toAssetHash, ccnotereq.amount);
//             // (7900, "6f65e468180df6d79c1465fd07312ac1c94da102a69feb3e7ec4a7a9b0f44b00"),
//             (79, EventSetting{  topics: vec![
//                                     H256::try_from("1a95c27c7cb22d2091f867137cb4d13c851b219adc23cf1af2e9d355eba66981").unwrap(), 
//                                     H256::try_from("6f65e468180df6d79c1465fd07312ac1c94da102a69feb3e7ec4a7a9b0f44b00").unwrap()],
//                                 contract:Address::try_from("0x37942a93c2978e5ce76cf057545718b1d6ec36ba").unwrap(),
//                                 filterid: 0,
//                                 startblock: U64::try_from_int(14478756_u64).unwrap(),
//             }),
//         ]
//         .iter()
//         .map(|(id, item) | (id.clone(), item.clone()))
//         // .copied()
//         .collect();

//         noteeventmap
//     });
// }

// Refer to https://github.com/polynetwork/docs/blob/master/config/README_TestNet.md 
// for definition of Chainid
pub static NOTEPROXYMAPPING: Lazy<Arc<HashMap<u64, Address>>> = Lazy::new(|| Arc::new({
    // TODO: Read these from a CSV?
    let chainproxymap: HashMap<u64, Address> = [
        // 0x Exchange Proxies
        (2, &std::env::var("ETH_NOTE_PROXY").unwrap()),      // Ethereum / Ropsten /Rinkeby "0x3512D865860627537C32027ed2CDBdC1882dD6A5"
        (79, &std::env::var("BSC_NOTE_PROXY").unwrap()),     // BSC "0x37942a93c2978e5ce76cf057545718b1d6ec36ba"
    ]
    .iter()
    .map(|(id, proxyaddr)| (id.clone(), Address::from_str(proxyaddr.clone()).unwrap()))
    .collect();

    chainproxymap
}));
