// connect to DB and return a Collection
// for now it's just a string

use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::{Client, Collection};

use crate::assetdef;
use assetdef::Asset;

pub async fn connect_to_db() -> Option<mongodb::Collection<Asset>> {
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await;

    match client {
        Ok(c) => {
            let database = c.database("sharks");
            let collection: mongodb::Collection<Asset> = database.collection("tiger");
            Some(collection)
        }
        Err(_e) => None,
    }
}
