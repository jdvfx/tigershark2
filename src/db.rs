// connect to DB and return a Collection (Option)
// for now the database and collection are hard-coded
//
use mongodb::Client;

use crate::assetdef;
use assetdef::Asset;

pub async fn connect_to_db(
    uri: String,
    db_name: String,
    coll_name: String,
) -> Option<mongodb::Collection<Asset>> {
    let client = Client::with_uri_str(&uri).await;
    match client {
        Ok(c) => {
            let database = c.database(&db_name);
            let collection: mongodb::Collection<Asset> = database.collection(&coll_name);
            Some(collection)
        }
        Err(_e) => None,
    }
}
