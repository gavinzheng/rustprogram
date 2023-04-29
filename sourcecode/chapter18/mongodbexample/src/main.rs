extern crate mongodb;
use mongodb::bson::doc;
use mongodb::{Client, options::ClientOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client_options = ClientOptions::parse("mongodb://47.104.153.34:14100").await?;
    // let client = Client::with_options(client_options).unwrap();
    let client = Client::with_uri_str("mongodb://47.104.153.34:14100").await?;
    let db_name = "mydatabase";
    let coll_name = "mycollection";

    create_collection(&client, db_name, coll_name).await;
    Ok(())
}

async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    db.create_collection(coll_name, None).await.unwrap();
}
async fn insert_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection(coll_name);

    let doc = doc! { "name": "Milly", "age": 16 };

    coll.insert_one(doc, None).await.unwrap();
}

async fn get_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection(coll_name);

    let filter = doc! {"name": "John"};

    let result = coll.find_one(Some(filter), None).await.unwrap();
    match result {
        Some(doc) => println!("{}", doc),
        None => println!("No document found"),
    }
}
fn delete_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection(coll_name);

    let filter = doc! {"name": "John"};
    coll.delete_one(filter, None).await.unwrap();
}
fn update_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection(coll_name);

    let filter = doc! {"name": "John"};
    let update = doc! {"$set": {"age": 35}};
    coll.update_one(filter, update, None).await.unwrap();
}