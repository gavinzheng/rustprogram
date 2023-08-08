// use std::env;
// use std::fs;
//use std::fs::File;
use std::str::FromStr;
// use std::time;

use web3::{
    ethabi::ethereum_types::U256,
    types::{Address, Bytes, TransactionParameters, /*TransactionReceipt, H256, H160*/},
    // futures::StreamExt,
    // confirm,
};
use web3::contract::{Contract,Options};
use web3::contract::tokens::Tokenize;

use crate::{Result};
use crate::api::*;
// use crate::utils::*;
use crate::zokcommand::*;
use crate::notejson::*;
use crate::config::*;

use warp::{http::StatusCode, reply, Reply};
// use secp256k1::SecretKey;
// use std::process::Command;

// use crate::config; // ::{CC_TRANS_STATUS_MAPPING};
use crate::CC_TRANS_STATUS_MAPPING;
use crate::api::{ CCTRANSSTATUS, CCTRANSItem};
use crate::utils::*;

// Name     :  deposit_ccnote_out
// 
// Function ： Deposit CC Note Handler
//      initiate a CC Transaction and keep tx for monitoring
//    
// Parameter: 
// 参数名	        参数说明	类型	备注
// pk1	            目标客户公钥1	uint256	
// pk2	            目标客户公钥1	uint256	
// amount	        资产转移数目	uint256	目前的测试资产，小数点后6位
// fromassethash	源资产合约地址	String	
// tochainid	    目标链ID	    uint64	
// encryptednote	资产说明	    String	
// fromchainid	    源资产链ID	    uint64	
// userid	        用户ID	        uint64	测试环境专用
//
// URL： 47.104.153.34:8554/depositccnoteout
//
// Return : ZKPCCResponse 
//      code : =0 Success, or Error code 
//      message : error message
//
pub async fn deposit_ccnote_out(
    req : DepositNoteRequest
) -> Result<impl Reply> {
    debug!("handling deposit note ... ");
    debug!(target: "deposit", "Start of DepositCCOut request");

    // Connection
    let http = if req.fromchainid == 2 {
        web3::transports::Http::new(&std::env::var("INFURA_ROPSTEN").unwrap()).unwrap()
    }else if req.fromchainid == 79 {
        web3::transports::Http::new(&std::env::var("BSC_ENDPOINT").unwrap()).unwrap()
    }else {
        debug!(target: "deposit", "Invalid Chain ID in deposit_ccnote_out handler from chain ID= {}", req.fromchainid);

        return     Ok(reply::with_status(
            reply::json(&ZKPCCResponse{code: 101,  data: "{}".to_string(), message: "Invalid Chain ID".to_string()}),
            StatusCode::OK,
        ));
    };
    let web3s = web3::Web3::new(http);
    let reqclone001 = req.clone();

    let noteproxyaddress = noteproxylookup(req.fromchainid);
    let noteproxy =
         Contract::from_json(web3s.eth(), noteproxylookup(req.fromchainid), 
                            include_bytes!("../abi/erc20ZKPNoteproxyPreStart.json")).unwrap();
        
    let mut accounts = web3s.eth().accounts().await.unwrap();
    accounts.push(crate::config::userlookup(req.userid).addr);

    println!("Accounts: {:?}", &accounts);
 
    let fromassethash= Address::from_str(&req.fromassethash).unwrap();
    let encryptednote: String = req.encryptednote.clone();
    let out_gas_estimate = noteproxy
    .estimate_gas(
        "depositCCOut",
        (
            fromassethash, req.tochainid, req.amount, req.pk1, req.pk2, encryptednote,
        ),
        accounts[0],
        Options {
            value: Some(U256::from_dec_str("0").unwrap()), // Some(U256::exp10(18).checked_div(20.into()).unwrap()),
            gas: Some(500_000.into()),
            ..Default::default()
        },
    )
    .await
    //.map_err(Into::into);
    .expect("Error");

    debug!(target: "deposit", "estimated gas amount: {}", out_gas_estimate);

    let gas_price = web3s.eth().gas_price().await.unwrap();
    debug!(target: "deposit", "gas price: {}", gas_price);
    
    let data = noteproxy
        .abi()
        .function("depositCCOut")
        .unwrap()
        .encode_input(
            &(
                fromassethash, req.tochainid, req.amount, req.pk1, req.pk2, req.encryptednote,
            )
            .into_tokens(),
    )
    .unwrap();
    // println!("TxHash: {}", data);

    let nonce = web3s
        .eth()
        .transaction_count(accounts[0], None)
        .await
        .unwrap();

    debug!(target: "deposit", "nonce: {}", nonce);         

    let transact_obj = TransactionParameters {
        nonce: Some(nonce),
        to: Some(noteproxyaddress),
        value: U256::from_dec_str("0").unwrap(), // U256::exp10(18).checked_div(50.into()).unwrap(), 
        gas_price: Some(gas_price),
        gas: out_gas_estimate*2,
        data: Bytes(data),
        ..Default::default()
    };
    debug!(target: "deposit", "transact_obj {:?}", transact_obj);

    let private_key = u256_to_secretkey(userlookup(req.userid).sk);
    let signed_transaction = web3s
        .accounts()
        .sign_transaction(transact_obj, &private_key)
        .await
        .unwrap();

    debug!(target: "deposit", "signed transaction: {:?}", signed_transaction);

    let result = web3s
        .eth()
        .send_raw_transaction(signed_transaction.raw_transaction)
        .await
        .unwrap();


    let curblockno = web3s.eth().block_number().await.unwrap().as_u64();
    let mut txmap = CC_TRANS_STATUS_MAPPING.lock().await;
    txmap.insert(
        result,
        CCTRANSItem{
            request : DepositNoteRequest{
                pk1: req.pk1, 
                pk2: req.pk2, 
                amount: req.amount, 
                tochainid : req.tochainid,
                fromassethash : req.fromassethash, 
                encryptednote : reqclone001.clone().encryptednote,      // Detail 
                fromchainid : req.fromchainid,                          // Deposit from Chain ID
                userid:0,
            },
            noteid: U256::from_dec_str("0").unwrap() ,
            notereq: U256::from_dec_str("0").unwrap(),
            rootdigest: U256::from_str_radix(&"0", 16).unwrap(),
            leaf : U256::from_str_radix(&"0x0", 16).unwrap(),
            pathdigest0: U256::from_str_radix(&"0x0000000000000000000000000000000000000000000000000000000000000000",16).unwrap(),
            pathdigest1: U256::from_str_radix(&"0",16).unwrap(),
            direction: [U256::from_dec_str("0").unwrap(),U256::from_dec_str("0").unwrap(),],
            status: CCTRANSSTATUS::Sent,
            fromtx: result,
            blockno: curblockno,
            fromaddress: Address::from_str(&std::env::var("ETH_ACCOUNT_ADDRESS").unwrap()).unwrap(),
            // to_Asset: Address::from_str("0x0").unwrap(),
        }
    );
    let len = txmap.len();
    debug!(target: "deposit", "Handler map length ={}", len);

    debug!(target: "deposit", "Transaction successful with hash: {:?}", result);
    info!(target: "deposit", "End of DepositCCOut request");


    Ok(reply::with_status(
        reply::json(&ZKPCCResponse{code: 0,  data: "{}".to_string(), message: "deposit_ccnote_out Method success".to_string()}),
        StatusCode::OK,
    ))
}

// Name     :  generateproof
// 
// Function ： Generate Zokrates Proof
//      After a CC Transaction is completed, generate zokrate proof for subsequent Spend
//    
// Parameter: 
// 参数名	        参数说明	类型	备注
// noteid	        源链资产ID uint256	
// userid	        用户ID	   uint64	测试环境专用
//
// URL： 47.104.153.34:8554/generateproof
//
// Return : ZKPCCResponse 
//      code : =0 Success, or Error code 
//      message : error message
//
pub async fn generateproof(
    req : GenerateProofRequest, 
) -> Result<impl Reply> {
    debug!(target:"zok","handling generate proof ... ");

    let note = match readnotejson(req.noteid.to_string()) {
        // Todo : Cannolize Error code and Error String
        Err(err)    => return  Ok(reply::with_status(reply::json(&ZKPCCResponse{code: 001,  data: "{}".to_string(),message: err.to_string()}),StatusCode::OK)),
        Ok(note )   => note,
    };
    debug!("pk1={}", note.request.pk1.to_string());

    // Todo: Error Check
    match zokcmd_computewitness(
        note.request.pk1, 
        note.request.pk2,
        U256::from_dec_str(&userlookup(req.userid).sk.to_string()).unwrap(),
        note.rootdigest, 
        note.leaf,
        note.pathdigest0, 
        note.pathdigest1,  
        note.direction[0], 
        note.direction[1]
    ){
        Ok(()) => (),
        Err(err)=>{ 
            // Todo : Cannolize Error code and Error String
            debug!("zokcmd_computewitness return error: {}", err);
            return  Ok(reply::with_status(reply::json(&ZKPCCResponse{code: 101,  data: "{}".to_string(), message: "Zokrates Compute witness Error".to_string()}),StatusCode::OK));
        }
    };
 
    match zokcmd_renameproof(note.noteid){
        Ok(()) => (),
        Err(err)=>{ 
            // Todo : Cannolize Error code and Error String
            debug!("zokcmd_renameproof return error: {}", err);
            return  Ok(reply::with_status(reply::json(&ZKPCCResponse{code: 102,  data: "{}".to_string(), message: "Zokrates Rename Proof Error".to_string()}),StatusCode::OK));
        }
    };
    // match zokcmd_generateproof(){
    //     Ok(()) => (),
    //     Err(err)=>{ 
    //         // Todo : Cannolize Error code and Error String
    //         debug!("zokcmd_generateproof return error: {}", err);
    //         return  Ok(reply::with_status(reply::json(&"{code: 002, message: \"Zokrates Generate Proof  Error\"}"),StatusCode::OK));
    //     }
    // };

    // Todo: renmae noteid proof

    Ok(reply::with_status(
        reply::json(&ZKPCCResponse{code: 0,  data: "{}".to_string(), message: "generateproof Method success".to_string()}),
        StatusCode::OK,
    ))
}

// Name     :  spend_ccnote
// 
// Function ： Spend CC Note Handler
//      Submit ZK proof with recipient address. If ZKP validation pass,then asset will be transferred to recipient
//    
// Parameter: 
// 参数名	        参数说明	    类型	    备注
// noteid	        源链资产ID	    uint256
// notereq	        跨链资产ID	    uint256	
// amount	        资产转移数目	uint256	    目前的测试资产，小数点后6位
// recipient	    接收地址	    Address	
// fromchainid	    源资产链ID	    uint64	
// userid	        用户ID	        uint64	    测试环境专用
//
// URL： 47.104.153.34:8554/spendccnote
//
// Return : ZKPCCResponse 
//      code : =0 Success, or Error code 
//      message : error message
//
// Prototype:  
//      function spendNote(uint[2] calldata a, uint[2][2] calldata b, uint[2] calldata c, uint[3] calldata input, 
//                            bytes32 notereq, uint256 amount, address recipient) external payable returns (bool){
//
pub async fn spend_ccnote(
    req : SpendNoteRequest
) -> Result<impl Reply> {
    debug!(target: "spend", "handle spend CC note ...");

    // Connection
    let http = if req.fromchainid == 2 {
        web3::transports::Http::new(&std::env::var("INFURA_ROPSTEN").unwrap()).unwrap()
    }else if req.fromchainid == 79 {
        web3::transports::Http::new(&std::env::var("BSC_ENDPOINT").unwrap()).unwrap()
    }else {
        debug!(target: "deposit", "Invalid Chain ID in spend_ccnote handler from chain ID= {}", req.fromchainid);
         // Todo : return OK with response
        web3::transports::Http::new(&std::env::var("INFURA_ROPSTEN").unwrap()).unwrap()
    };
    let web3s = web3::Web3::new(http);

    let noteproxy =
        Contract::from_json(web3s.eth(), 
                noteproxylookup(req.fromchainid), 
                include_bytes!("../abi/erc20ZKPNoteproxyPreStart.json")).unwrap();

    let mut accounts = web3s.eth().accounts().await.unwrap();
    accounts.push(userlookup(req.userid).addr);
    println!("Accounts: {:?}", &accounts);

    let noteproxyaddress = noteproxylookup(req.fromchainid);
    // if req.fromchainid == 2 {
    //     web3::transports::Http::new(&std::env::var("INFURA_ROPSTEN").unwrap()).unwrap()
    // }else if req.fromchainid == 79 {
    //     web3::transports::Http::new(&std::env::var("BSC_ENDPOINT").unwrap()).unwrap()
    // }else {
    //     debug!(target: "deposit", "Invalid Chain ID in spend_ccnote handler from chain ID= {}", req.fromchainid);
    //      // Todo : return OK with response
    //     web3::transports::Http::new(&std::env::var("INFURA_ROPSTEN").unwrap()).unwrap()
    // };
    // Address::from_str(&std::env::var("BSC_NOTE_PROXY").unwrap()).unwrap();

    ///////////////////////////////////////////////////////////////////
    let noteproof = match readproofjson(format!("{}-proof.json", req.noteid)){
        Ok(noteproof) => noteproof,
        Err(e) =>{
            debug!(target:"spend", "readproofjson error in spend_ccnote()");
            return  Ok(reply::with_status(
                reply::json(&ZKPCCResponse{code: 7 ,  data: "{}".to_string(), message: format!("readproofjson error in spend_ccnote():{}", e)}),
                StatusCode::OK,
            ));
        }
    };

    let note= match readnotejson(format!("{}", req.noteid)){
        Ok(note) => note,
        Err(e) =>{
            debug!(target:"spend", "readnotejson error in spend_ccnote()");
            return  Ok(reply::with_status(
                reply::json(&ZKPCCResponse{code: 8 , data: "{}".to_string(),message: format!("readnotejson error in spend_ccnote():{}", e)}),
                StatusCode::OK,
            ));
        }
    };

    // let inputconvert =noteproof.inputs.iter().map(|x| U256::from_str(&x).unwrap()).collect::<Vec<U256>>();
    // debug!("inputconvert length = {}", inputconvert.len());

    // let mut bytesinput : [U256;19]=[U256::from_str("0").unwrap();19];
    // for k in 0 .. inputconvert.len(){
    //     bytesinput[k]= inputconvert[k];
    // }
    let mut bytesreq : [u8;32]=[0;32];
    req.notereq.to_big_endian(&mut bytesreq);
    // let bytesreqvec : Vec<u8>= bytesreq.iter().map(|x| x.clone()).collect();//::<Vec<u8>>();

    let out_gas_estimate = noteproxy
        .estimate_gas(
            "spendNote",
            (
                [U256::from_str(&noteproof.proof.a[0]).unwrap(), U256::from_str(&noteproof.proof.a[1]).unwrap()], 
                [[U256::from_str(&noteproof.proof.b[0][0]).unwrap(), U256::from_str(&noteproof.proof.b[0][1]).unwrap()],
                    [U256::from_str(&noteproof.proof.b[1][0]).unwrap(), U256::from_str(&noteproof.proof.b[1][1]).unwrap()]],
                [U256::from_str(&noteproof.proof.c[0]).unwrap(), U256::from_str(&noteproof.proof.c[1]).unwrap()],
                [note.request.pk1, note.request.pk2, note.rootdigest, note.pathdigest0,
                U256::from_str("1").unwrap(), ], 
                bytesreq, 
                req.amount, 
                req.recipient
            ),
            accounts[0],
            Options {
                value: Some(U256::from_dec_str("0").unwrap()), // Some(U256::exp10(18).checked_div(20.into()).unwrap()),
                gas: Some(500_000.into()),
                ..Default::default()
            },
        )
        .await
        .expect("Error");

    debug!(target: "spend", "estimated gas amount: {}", out_gas_estimate);

    let gas_price = web3s.eth().gas_price().await.unwrap();
    debug!(target: "spend", "gas price: {}", gas_price);
    
    let data = noteproxy
        .abi()
        .function("spendNote")
        .unwrap()
        .encode_input(
            &(
                [U256::from_str(&noteproof.proof.a[0]).unwrap(), U256::from_str(&noteproof.proof.a[1]).unwrap()], 
                [[U256::from_str(&noteproof.proof.b[0][0]).unwrap(), U256::from_str(&noteproof.proof.b[0][1]).unwrap()],
                    [U256::from_str(&noteproof.proof.b[1][0]).unwrap(), U256::from_str(&noteproof.proof.b[1][1]).unwrap()]],
                [U256::from_str(&noteproof.proof.c[0]).unwrap(), U256::from_str(&noteproof.proof.c[1]).unwrap()],
                [note.request.pk1, note.request.pk2, note.rootdigest, note.pathdigest0,
                U256::from_str("1").unwrap(), ], 
                bytesreq, 
                req.amount, 
                req.recipient
            )
            .into_tokens(),
    )
    .unwrap();
    // debug!("TxHash: {}", data);

    let nonce = web3s
        .eth()
        .transaction_count(accounts[0], None)
        .await
        .unwrap();

    debug!(target: "spend", "nonce: {}", nonce);

    let transact_obj = TransactionParameters {
        nonce: Some(nonce),
        to: Some(noteproxyaddress),
        value: U256::exp10(18).checked_div(50.into()).unwrap(),
        gas_price: Some(gas_price),
        gas: out_gas_estimate,
        data: Bytes(data),
        ..Default::default()
    };
    debug!(target: "spend", "transact_obj {:?}", transact_obj);

    // let private_key = SecretKey::from_str(&env::var("ETH_PRIVATE_KEY").unwrap()).unwrap();
    // let mut bytespriv: [u8;32]=[0;32];
    // userlookup(req.userid).sk.to_big_endian(& mut bytespriv);
    // let private_key = SecretKey::from_slice(&bytespriv).unwrap();
    let private_key = u256_to_secretkey(userlookup(req.userid).sk);
    let signed_transaction = web3s
        .accounts()
        .sign_transaction(transact_obj, &private_key)
        .await
        .unwrap();

    debug!(target: "spend", "signed transaction: {:?}", signed_transaction);

    let result = web3s
        .eth()
        .send_raw_transaction(signed_transaction.raw_transaction)
        .await
        .unwrap();

    debug!(target: "spend", "Transaction successful with hash: {:?}", result);

    info!(target: "spend", "End of spendNote request");


    Ok(reply::with_status(
        reply::json(&ZKPCCResponse{code: 0 , data: "{}".to_string(), message: "Spend CC Note Success".to_string()}),
        StatusCode::OK,
    ))
}

// Name     :  balance
// 
// Function ： User Balance Handler
//      Return the CC Assets of a user specified by userid
//    
// Parameter: 
// 参数名	        参数说明	    类型	    备注
// userid	        用户ID	        uint64	    测试环境专用
//
// URL： 47.104.153.34:8554/balance
//
// Return : ZKPCCResponse 
//      code : =0 Success, or Error code 
//      message : error message
//      data: json of BalanceResponse 
//
pub async fn balance(
    req : BalanceRequest
) -> Result<impl Reply> {
    info!("handle balance request ...");
    
    let retdata = match readdirnotejson(req.userid){
        Err(err)   => { 
            println!("readdirnotejson Fail: Error={}", err);
            info!("readdirnotejson: {} ", err);

            return Ok(reply::with_status(
                reply::json(&ZKPCCResponse{code: 009 , data: "{}".to_string(), message: err.to_string()}),
                StatusCode::OK,
            ));
        }
        Ok(retdata)   => {
            retdata
        }

    };

    Ok(reply::with_status(
        reply::json(&ZKPCCResponse{code: 0 , data: retdata, message: "Balance Success".to_string()}),
        StatusCode::OK,
    ))
}

pub async fn load_note(
    req : LoadNoteRequest
) -> Result<impl Reply> {
    info!("handle load note request ...");

    let mut transmap = CC_TRANS_STATUS_MAPPING.lock().await;
    let noteid = U256::from_str(&req.noteid).unwrap();
    match readnotejson(req.noteid.to_string()) {
        Err(err)   => { 
            println!("Can not read note: {} Error={}", noteid, err);
            info!("Can not read note: {} ", noteid);

            return Ok(reply::with_status(
                reply::json(&ZKPCCResponse{code: 006 , data: "{}".to_string(), message: err.to_string()}),
                StatusCode::OK,
            ));
        }
        Ok(note)   => {
            transmap.insert(
                note.fromtx, note
            );
        }
    };

    // writejson(serde_json::to_string_pretty(&item).unwrap().to_string().as_bytes(), "52138252125203178713318795450414024069006896720816266979072757700350127272275".to_string());
    // writejson(serde_json::to_string_pretty(&item).unwrap().to_string().as_bytes(), "test002".to_string());
     
    info!("End of load note request ...");
    
    Ok(reply::with_status(
        reply::json(&ZKPCCResponse{code: 0 ,  data: "{}".to_string(), message: "Load Note Success".to_string()}),
        StatusCode::OK,
    ))
}