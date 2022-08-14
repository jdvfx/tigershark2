// connect to DB and return a Collection (Option)
// uri, db_name, collection are environment variables
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
        Err(..) => None,
    }
}
