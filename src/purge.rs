#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use bson::{de::Error, raw::ErrorKind};
use mongodb::bson::doc;
pub mod parse_args;
use errors::{exit_or_panic, CliOutput};
use parse_args::CommandType;

mod assetdef;
pub mod db;
pub mod errors;
pub mod utils;
use assetdef::Asset;
use std::env;

#[tokio::main]
async fn main() {
    let cli_output: CliOutput;
    let uri = "mongodb://localhost:27017";
    let db_name = "sharks";
    let collection_name = "tiger";

    let collection = db::connect_to_db(uri, db_name, collection_name);
    match collection.await {
        Some(coll) => {
            // let s = purge(coll);

            let filter: bson::Document = doc! { "versions": {"$elemMatch": { "status": "Purge"}}};
            let mut cursor = coll.find(filter.clone(), None).await.unwrap();
            while cursor.advance().await.unwrap() {
                let c = cursor.deserialize_current();
                let name = &c.as_ref().unwrap().name;
                for version in c.as_ref().unwrap().versions.iter() {
                    let status = &version.status;
                    println!("{:?} {:?} {:?}", name, version, status);
                }
                // println!("\n{:?}", c);
            }
        }
        None => println!("no collections"),
    }
}

// async fn purge(coll: mongodb::Collection<Asset>) -> Result<String, ErrorKind> {
//     let filter: bson::Document = doc! { "versions": {"$elemMatch": { "status": "Purge"}}};
//     let mut cursor = coll.find(filter.clone(), None).await?;
//     while cursor.advance().await? {
//         let c = cursor.deserialize_current();
//         println!("\n{:?}", c);
//     }
//     Ok("S".to_string())
// }
