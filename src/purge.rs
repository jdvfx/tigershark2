#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use std::fs::File;
use std::io::prelude::*;

use bson::{de::Error, raw::ErrorKind};
use mongodb::bson::doc;
pub mod parse_args;
use errors::{exit_or_panic, CliOutput};
use parse_args::CommandType;

mod assetdef;
pub mod db;
pub mod errors;
pub mod utils;
use assetdef::{Asset, AssetStatus};
use std::env;

#[tokio::main]
async fn main() {
    let output_file = "files_to_purge";
    let cli_output: CliOutput;

    // Connect to DB, needs 3 env variables: MONGODB_URI, MONGODB_DB, MONGODB_COLL
    let uri: &str = &env::var("MONGODB_URI").expect("MONGODB_URI environment var not set!");
    let db_name: &str = &env::var("MONGODB_DB").expect("MONGODB_DB environment var not set!");
    let collection_name: &str =
        &env::var("MONGODB_COLL").expect("MONGODB_COLL environment var not set!");
    //
    let collection = db::connect_to_db(uri, db_name, collection_name);
    match collection.await {
        Some(coll) => {
            let filter: bson::Document = doc! { "versions": {"$elemMatch": { "status": "Purge"}}};
            let mut cursor = coll.find(filter.clone(), None).await.unwrap();
            let mut output = File::create(&output_file).unwrap();
            while cursor.advance().await.unwrap() {
                let c = cursor.deserialize_current();
                // let name = &c.as_ref().unwrap().name;
                for version in c.as_ref().unwrap().versions.iter() {
                    match &version.status {
                        AssetStatus::Purge => {
                            let datapath = &version.datapath;
                            println!("{:?}", &datapath);
                            writeln!(output, "{}", &datapath).ok();
                        }
                        _ => (),
                    }
                }
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
