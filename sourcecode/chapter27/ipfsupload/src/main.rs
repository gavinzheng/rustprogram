// use pinata_sdk::PinataApi;
use pinata_sdk::{ApiError, PinataApi, PinByFile};

#[tokio::main]
async fn main() {
    let api = PinataApi::new("30c3ceba920671dae1b7", "d64108eeecb62fcd63ef31507abc486e41d01277f8ec250dd9f815d26705fd77").unwrap();

    // test that you can connect to the API:
    // let result = api.test_authentication().await;
    // if let Ok(_) = result {
    // // credentials are correct and other api calls can be made
    //     println!("ipfsupload test successful");
    // }

    let result = api.pin_file(PinByFile::new("example.txt")).await;

    if let Ok(pinned_object) = result {
        let hash = pinned_object.ipfs_hash;
        println!("ipfsupload hash {:?}", hash);
    }else{
        println!("ipfsupload failed");
    }
}
