// use serde_derive::{Deserialize, Serialize};
// use std::path::Path;
use std::fs;
use std::fs::File;
//use std::fmt::Write;
// use std::io::Read;
 use std::io::Write;
// use serde::{Deserialize, Serialize};
use crate::api::*;
// use serde_json::{json, Value};
// use serde_json::{json};
use web3::types::U256;

pub fn writejson(content: &[u8], name: String) -> std::io::Result<()> {
    let mut f = File::create(
                        format!(
                            "./{}/{}{}",
                            std::env::var("NOTEDIR").expect("expect notedir"), 
                            std::env::var("NOTEPREFIX").expect("expect prefix"),name))?;
    
    f.write_all(content)?; 
    f.sync_all()?;
    Ok(())
}

pub fn readjson(name: String) -> std::io::Result<String>  {
    debug!("readjson: json file full name= {}",name);
    let s = fs::read_to_string(name)?;
    Ok(s)
}

pub fn readnotejson(name: String) -> std::io::Result<CCTRANSItem> {
    debug!("file name ={}", format!("./{}/{}{}", std::env::var("NOTEDIR").expect("expect notedir"),std::env::var("NOTEPREFIX").expect("expect prefix"),name));
    let content = readjson(
        format!(
            "./{}/{}{}",
            std::env::var("NOTEDIR").expect("expect notedir"), 
            std::env::var("NOTEPREFIX").expect("expect prefix"),name))?; 
    debug!("Read Note: {} , Note={}", name, content);
    let note: CCTRANSItem = serde_json::from_str(&content)?; // .unwrap();
    
    Ok(note)
}

pub fn readproofjson(name: String) -> std::io::Result<SpendProofJSON> {
    let content1 = readjson(
        format!(
            "./{}/{}",
            std::env::var("NOTEDIR").expect("expect notedir"), 
            name))?;
    let cfg: SpendProofJSON = serde_json::from_str(&content1)?;
    
    debug!("proofa={0}, \n proofb=={1}, \n proofc={2:?},\n inputs={3:?},\n inputs={4:?},\n inputs=={5:?}",
            cfg.proof.a[0], cfg.proof.b[0][0], cfg.proof.c[0], cfg.inputs[0], cfg.inputs[1], cfg.inputs[2] );  
    Ok(cfg)
}

pub fn readdirnotejson(userid: u64) -> std::io::Result<String> {
    let mut totalamount = U256::from_dec_str("0").unwrap();
    let mut vecdetails : Vec<CCTRANSItem> = Vec::new();

    let paths = fs::read_dir(format!("./{}/", std::env::var("NOTEDIR").expect("expect notedir"))).unwrap();
    for path in paths {
        debug!("Name: {}", path.as_ref().unwrap().path().display());
        if !path.as_ref().unwrap().path().to_str().unwrap().contains(&std::env::var("NOTEPREFIX").expect("expect prefix")){
            continue;
        }
        let content = readjson(path.as_ref().unwrap().path().display().to_string())?;

        let note: CCTRANSItem = serde_json::from_str(&content)?; 
        if note.status == CCTRANSSTATUS::Transferred {
            totalamount = totalamount + note.request.amount; 

            vecdetails.push( 
                CCTRANSItem{
                    request : DepositNoteRequest{
                        pk1: note.request.pk1,
                        pk2: note.request.pk2,
                        amount: note.request.amount,
                        tochainid : note.request.tochainid,
                        fromassethash : note.request.fromassethash,
                        encryptednote : note.request.encryptednote,     // Detail 
                        fromchainid : note.request.fromchainid,            // Deposit from Chain ID
                        userid: userid,
                    },
                    noteid: note.noteid,
                    notereq: note.notereq,
                    rootdigest: note.rootdigest,
                    leaf : note.leaf,
                    pathdigest0: note.pathdigest0,
                    pathdigest1: note.pathdigest0,
                    direction: [note.direction[0],note.direction[1],],
                    status: note.status,
                    fromtx: note.fromtx,
                    blockno: note.blockno,
                    fromaddress: note.fromaddress,

                    // toAssethash:H256::from_str("0").unwrap(),
                }
            );
        }

    }

    let sdata = serde_json::to_string(&BalanceResponse{totalamount: totalamount, length: vecdetails.len() as u64, detail: vecdetails});
    if sdata.is_err() {
        debug!("Error, failed to serialize structure: {}", sdata.unwrap_err());
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "serde_json Serialize error in readdirnotejson"));
    }

    let retdata = sdata.unwrap();
    Ok(retdata)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readjson() {
        let content = readjson("tmp".to_string());
        let cfg: CCTRANSItem = serde_json::from_str(&content.unwrap()).unwrap();
        println!("rtDigest={0:x}, \n _leaf=={1:x}, \n direction0={2:?},\n direction1={3:?},\n digest1={4:x},\n digest2=={5:x}",
                cfg.rootdigest, cfg.leaf, cfg.direction[0], cfg.direction[1], cfg.pathdigest0, cfg.pathdigest1 );  
    }
}


