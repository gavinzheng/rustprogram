use serde_derive::{Deserialize, Serialize};
// use crate::errors::{AppError};
// use crate::data_access::DBAccessManager;
// use serde::Serialize;
use web3::types::{Address, H256, U256};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ZKPCCResponse {
    pub code : u32,
    pub data : String,
    pub message : String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BalanceResponse{
    pub totalamount: U256,
    pub length : u64,
    pub detail : Vec<CCTRANSItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DepositNoteRequest {
    pub pk1: U256,
    pub pk2: U256,
    pub amount: U256,
    pub tochainid : u64,            // Deposit to Chain ID
    pub fromassethash : String,     // From Asset Hash
    pub encryptednote : String,     // Detail 
    pub fromchainid : u64,          // Deposit from Chain ID
    pub userid: u64,               // userid 
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum CCTRANSSTATUS {
    Sent = 1,          // transaction send 
    Included =2,       // mined
    Transferred =3 ,   // Transferred to the other chain
    Complete =4,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CCTRANSItem {
    pub request : DepositNoteRequest,
    pub noteid: U256 ,
    pub notereq: U256,

    pub rootdigest: U256,
    pub leaf : U256,
    pub pathdigest0: U256,
    pub pathdigest1: U256,
    pub direction: [U256;2],

    // pub privkey: U256,

    pub status: CCTRANSSTATUS,
    pub fromtx: H256,
    pub fromaddress: Address, 
    // pub to_Asset: Address,     // To Asset Hash

    pub blockno:u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GenerateProofRequest {
    // pub userid: U256,
    pub noteid: String,
    pub userid: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BalanceRequest {
    pub userid: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoadNoteRequest {
    pub userid: String,
    pub noteid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpendNoteRequest {
    // pub calldataa : [U256; 2],
    // pub calldatab : [[U256; 2]; 2],
    // pub calldatac : [U256;2],
    // pub input : [U256;3],
    pub noteid : U256,
    pub notereq : U256,
    pub amount : U256,
    pub recipient : Address, 
    pub fromchainid : u64,          // Spend from Chain ID
    pub userid : u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpendProof{
    pub proofa : [U256;2],
    pub proofb : [[U256;2];2],
    pub proofc : [U256;2],
    pub inputs : [U256;19],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpendProofJSON{
    pub proof : ProofZOK,
    // pub proofb : [[U256;2];2],
    // pub proofc : [U256;2],
    pub inputs : [String;19],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofZOK{
    pub a : [String;2],
    pub b : [[String;2];2],
    pub c : [String;2],
}

// {
//     "proof": {
//       "a": [
//         "0x0d65cfd70a830e08da51129d7b09bae37a065839415a11de43312ddea42d2e2c",
//         "0x2b5cbb8c826086cca128ebdad908135a65d3af6ab89589bdff005e8d0d986fe5"
//       ],
//       "b": [
//         [
//           "0x035a09ee96a7948b1138248be99bdce7ffac2ed03d0b8a962e127072966d198f",
//           "0x00b65269d6686efc53dd8d2ed47081db49553d7a5f68fc186707ecfa59ad39e8"
//         ],
//         [
//           "0x2f2d71e58b0dfbe2db4e29b08be921f061505034504c6071a63c602296fb4818",
//           "0x26c6b5b3f95741a58f43940e1c63ec13f74b4f023186b013c424b0eb7b745d2c"
//         ]
//       ],
//       "c": [
//         "0x24f1776f4e64e3ac731c34606bffaa88a963adf991717f7dd7366619e362532d",
//         "0x000e5120aae7c19e90273f30922457f581a8ba0331d25462df6bcadc9feaffbe"
//       ]
//     },
//     "inputs": [
//       "0x26d264c0f94c7089a9be0782aba859f2977447f023085af54e02f0e1039fe3ce",
//       "0x07bcc180572b26488368240bb84114cf42eed9e708a03f6f431555fd8e135092",
//       "0x00000000000000000000000000000000000000000000000000000000feb65c66",
//       "0x000000000000000000000000000000000000000000000000000000005819c318",
//       "0x000000000000000000000000000000000000000000000000000000009bfcd044",
//       "0x000000000000000000000000000000000000000000000000000000006621cf1d",
//       "0x0000000000000000000000000000000000000000000000000000000099a70161",
//       "0x00000000000000000000000000000000000000000000000000000000bdc65efb",
//       "0x000000000000000000000000000000000000000000000000000000009d5ffde3",
//       "0x00000000000000000000000000000000000000000000000000000000c19d3a03",
//       "0x0000000000000000000000000000000000000000000000000000000000000000",
//       "0x0000000000000000000000000000000000000000000000000000000000000000",
//       "0x0000000000000000000000000000000000000000000000000000000000000000",
//       "0x0000000000000000000000000000000000000000000000000000000000000000",
//       "0x0000000000000000000000000000000000000000000000000000000000000000",
//       "0x0000000000000000000000000000000000000000000000000000000000000000",
//       "0x0000000000000000000000000000000000000000000000000000000000000000",
//       "0x0000000000000000000000000000000000000000000000000000000000000000",
//       "0x0000000000000000000000000000000000000000000000000000000000000001"
//     ]
//   }
