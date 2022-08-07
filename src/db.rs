// connect to DB and return a Collection (Option)
// for now the database and collection are hard-coded
//
use mongodb::Client;

use crate::assetdef;
use assetdef::Asset;

pub async fn connect_to_db(
    uri: &str,
    db_name: &str,
    coll_name: &str,
) -> Option<mongodb::Collection<Asset>> {
    let client = Client::with_uri_str(uri).await;
    match client {
        Ok(c) => {
            let database = c.database(db_name);
            let collection: mongodb::Collection<Asset> = database.collection(coll_name);
            Some(collection)
        }
        Err(_e) => None,
    }
}
