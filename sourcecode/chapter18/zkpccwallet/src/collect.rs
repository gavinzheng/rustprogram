
C:\dev\rust\api-master\src\lib.rs
#[cfg(feature = "remote-run")]
fn read_conf<T: serde::Deserialize, P: AsRef<Path>>(path: P) -> error::Result<T> {
    let mut fh = fs::File::open(&path)?;
    let mut json = String::new();
    fh.read_to_string(&mut json)?;
    Ok(serde_json::from_str(&json)?)
}

#[cfg(all(test, feature = "remote-run"))]
fn write_conf<C: serde::Serialize, P: AsRef<Path>>(conf: C, path: P) -> error::Result<()> {
    let json = serde_json::to_string(&conf)?;
    let mut fh = fs::File::create(&path)?;
    fh.write_all(json.as_bytes())?;
    Ok(())
}

C:\dev\rust\bip39\rust-bip39-master\src\lib.rs
for vector in &vectors {
    let entropy = Vec::<u8>::from_hex(&vector.0).unwrap();
    let mnemonic_str = vector.1;
    let passphrase = vector.2;
    let seed = Vec::<u8>::from_hex(&vector.3).unwrap();


    fn u32_to_array_be(val: u32) -> [u8; 4] {
        debug_assert_eq!(::std::mem::size_of::<u32>(), 4); // size_of isn't a constfn in 1.22
    
        let mut res = [0; 4];
        for i in 0..4 {
            res[i] = ((val >> (4 - i - 1)*8) & 0xff) as u8;
        }
        res
    }


    pub(crate) fn sha256_first_byte(input: &[u8]) -> u8 {
        sha2::Sha256::digest(input).as_ref()[0]
    }


    C:\dev\rust\MPCSamples\important\STIKO\wallet\ethane-main\ethane\tests\eth.rs

    C:\dev\rust\MPCSamples\important\STIKO\wallet\ethane-main\ethane\src\rpc\eth.rs
    
    fn test_eth_new_filter() {
        let mut client = ConnectionWrapper::new_from_env(None);
        let address = create_account(&mut client).1;
        let (contract_address, _) = deploy_contract(
            &mut client,
            address,
            &Path::new(TEST_CONTRACT_PATH),
            TEST_CONTRACT_NAME,
        );
        let topic = keccak(b"Solution(uint256)");
        let filter = Filter {
            from_block: Some(BlockParameter::Earliest),
            to_block: Some(BlockParameter::Latest),
            address: Some(ValueOrVec::Value(contract_address)),
            topics: Some(vec![Some(ValueOrVec::Value(
                H256::try_from(&topic).unwrap(),
            ))]),
        };
        rpc_call_test_some(&mut client, rpc::eth_new_filter(filter));
    }

    fn test_eth_get_filter_changes_block_filter() {
        let mut client = ConnectionWrapper::new_from_env(None);
        let tx = TransactionRequest {
            from: create_account(&mut client).1,
            to: Some(create_account(&mut client).1),
            value: Some(U256::zero()),
            ..Default::default()
        };
        let filter_id = client.call(rpc::eth_new_block_filter()).unwrap();
        let tx_hash = client.call(rpc::eth_send_transaction(tx)).unwrap();
        wait_for_transaction(&mut client, tx_hash);
        rpc_call_test_some(&mut client, rpc::eth_get_filter_changes(filter_id));
    }
    
    


    // let mut bytespriv: [u8;32]=[0;32];
    // // let strpriv = USERMAPPING
    // //                 .get(&req.userid)
    // //                 .unwrap()
    // //                 .clone(); 
    // crate::config::userlookup(req.userid).sk.to_big_endian(& mut bytespriv);
    // let private_key = SecretKey::from_slice(&bytespriv).unwrap();
    // debug!("priv key={}", private_key);

    // accounts.push(H160::from_str(&userlookup(req.userid).addr.to_string()).unwrap());
    //let inputconvert: Vec<U256> = noteproof.inputs.iter().map(|x| U256::from_str(&x[..]).unwrap()).collect();

// pub fn read_conf<T: serde::de::Deserialize, P: AsRef<Path>>(path: P) -> Result<T> {
//     let mut fh = fs::File::open(&path)?;
//     let mut json = String::new();
//     fh.read_to_string(&mut json)?;
//     Ok(serde_json::from_str(&json)?)
// }

// pub fn write_conf<C: serde::Serialize, P: AsRef<Path>>(conf: C, path: P) -> Result<()>{
//     let json = serde_json::to_string(&conf)?;
//     let mut fh = fs::File::create(&path)?;
//     fh.write_all(json.as_bytes())?;
//     Ok(())
// }

pub fn writejson(content: &[u8], name: String) -> std::io::Result<()> {
    //write_conf(obj, "./aa").expect("write json file error");
    // serde_json::to_writer(&File::create("./tmp"), &data);
    // let mut f = File::create(name)?; // .expect("Unable to create file")?;
    // let mut f = File::create(format!(".\\{}\\{}{}", std::env::var("NOTEDIR").expect("expect notedir"), std::env::var("NOTEPREFIX").expect("expect prefix"),name))?; // .expect("Unable to create file")?;
    let mut f = File::create(
                        format!(
                            "./{}/{}{}",
                            //std::env::var("NOTEPATHTEMPLATE").expect("expect NOTEPATHTEMPLATE"), 
                            std::env::var("NOTEDIR").expect("expect notedir"), 
                            std::env::var("NOTEPREFIX").expect("expect prefix"),name))?; // .expect("Unable to create file")?;
    
    f.write_all(content)?; // .expect("Unable to write data");
    f.sync_all()?;
    Ok(())
}

pub fn readjson(name: String) -> std::io::Result<String>  {
    // let path = concat!(env!("CARGO_MANIFEST_DIR"), "/config.json");
    // let notedir = std::env::var("NOTEDIR").expect("expect notedir");
    // let fullpath = format!("./{}/{}", std::env::var("NOTEDIR").expect("expect notedir"),name);
    // let s = fs::read_to_string(format!("../{}/{}{}", std::env::var("NOTEDIR").expect("expect notedir"), std::env::var("NOTEPREFIX").expect("expect prefix"),name))?;
    debug!("readjson: json file full name= {}",name);
    let s = fs::read_to_string(name)?;
    Ok(s)
}

// pub fn writenote(name : String, obj: CCTRANSItem) -> std::io::Result<()> {
//     let data = json!(obj);
//     serde_json::to_string_pretty(&data).unwrap();
//     writejson(serde_json::to_string_pretty(&data).unwrap().to_string().as_bytes(), name)?; 
//     // debug!("write Note: {} , Note={}", name, obj);
    
//     Ok(())
// }

// let data = json!(obj);
// //write_conf(obj, "./aa").expect("write json file error");
// // serde_json::to_writer(&File::create("./tmp"), &data);

// serde_json::to_string_pretty(&data).unwrap();
// writejson(serde_json::to_string_pretty(&data).unwrap().to_string().as_bytes(), "aa"); 
// // let mut f = File::create("./tmp").expect("Unable to create file");
// // f.write_all(data.to_string().as_bytes()).expect("Unable to write data");















    