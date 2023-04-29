use chrono::{TimeZone, Utc};
use mongodb::bson::{self, doc, Bson, DateTime};
use serde::{Deserialize, Serialize};
// use std::env;
use std::error::Error;
use tokio;

// We can use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug)]
struct Teacher {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<bson::oid::ObjectId>,
    title: String,
    year: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    // let client_uri =
    //     env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    let client = mongodb::Client::with_uri_str("mongodb://47.104.153.34:14100").await?;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    // Get the 'teachers' collection from the 'sample_mflix' database:
    let teachers = client.database("mydb").collection("teachers");

    let new_doc = doc! {
        "title": "Prof. Milly",
        "year": 2020,
        "desc": "Bachelor degree from  University XXX, master degree from University , Doctor from University ZZZ.",
        "released":   bson::DateTime::builder().year(1995).month(5).day(6).minute(1).millisecond(23).build()?, //Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),  //DateTime(Utc::now()),// 
    };
    println!("New Document: {}", new_doc);
    let insert_result = teachers.insert_one(new_doc.clone(), None).await?;
    println!("New document ID: {}", insert_result.inserted_id);

    // Look up one document:
    let teacher = teachers
        .find_one(
            doc! {
                "title": "Prof. Milly"
            },
            None,
        )
        .await?
        .expect("Can not find the specified teacher.");
    println!("teacher: {}", teacher);
    let title = teacher.get_str("title")?;
    // -> "Parasite"
    println!("teacher Title: {}", title);

    let teacher_json: serde_json::Value = Bson::from(teacher).into();
    println!("JSON: {}", teacher_json);

    // Update the document:
    let update_result = teachers
        .update_one(
            doc! {
                "_id": &insert_result.inserted_id,
            },
            doc! {
                "$set": { "year": 2015 }
            },
            None,
        )
        .await?;
    println!("Updated {} documents", update_result.modified_count);

    // Look up the document again to confirm it's been updated:
    let teacher = teachers
        .find_one(
            doc! {
                "_id": &insert_result.inserted_id,
            },
            None,
        )
        .await?
        .expect("Can not find the specified teacher.");
    println!("Updated teacher: {}", &teacher);

    // Delete all documents for Prof. Milly :
    let delete_result = teachers
        .delete_many(
            doc! {
                "title": "Prof. Milly"
            },
            None,
        )
        .await?;
    println!("Deleted {} documents", delete_result.deleted_count);

    if let Ok(title) = new_doc.get_str("title") {
        println!("title: {}", title);
    } else {
        println!("Can not find the specified teacher.");
    }

    // Initialize struct to be inserted:
    let assocprof_alice = Teacher {
        id: None,
        title: "Assoc Prof Alice".to_owned(),
        year: 2019,
    };

    // Convert `captain_marvel` to a Bson instance:
    let serialized_prof = bson::to_bson(&assocprof_alice)?;
    let document = serialized_prof.as_document().unwrap();

    // Insert into the collection and extract the inserted_id value:
    let insert_result = teachers.insert_one(document.to_owned(), None).await?;
    let alice_id = insert_result
        .inserted_id
        .as_object_id()
        .expect("Retrieved _id should have been of type ObjectId");
    println!("Assoc Prof Alice document ID: {:?}", alice_id);

    // Retrieve Assoc Prof Alice from the database, into a teacher struct:
    // Read the document from the teachers collection:
    let loaded_teacher = teachers
        .find_one(Some(doc! { "_id":  alice_id.clone() }), None)
        .await?
        .expect("Document not found");

    // Deserialize the document into a teacher instance
    let loaded_teacher_struct: Teacher = bson::from_bson(loaded_teacher.into())?;
    println!("Teacher loaded from collection: {:?}", loaded_teacher_struct);

    // Delete Assoc Prof Alice from MongoDB:
    teachers
        .delete_one(doc! {"_id": alice_id.to_owned()}, None)
        .await?;
    println!("Assoc Prof Alice record has been deleted.");

    Ok(())
}
