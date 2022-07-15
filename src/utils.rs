use crate::errors::{CliOutput, Status};
use crate::parse_args::Asset;

// CRUD functions

use mongodb::{bson::doc, Client, Collection};

pub async fn create(collection: mongodb::Collection<Asset>, asset: Asset) -> CliOutput {
    // > required:
    // asset_name, location, source

    // find doc from name and location
    let cursor = collection
        .find_one(Some(doc! { "name": &asset.name }), None)
        .await;

    match cursor {
        Ok(c) => match &c {
            Some(c) => {
                // Asset found in the DB
                CliOutput {
                    status: Status::Ok,
                    output: "Asset already exists".to_owned(),
                }
            }
            None => {
                // Asset not found in DB, try to insert it
                let insert_result = collection.insert_one(&asset, None).await;
                match insert_result {
                    Ok(..) => CliOutput {
                        status: Status::Ok,
                        output: "Asset inserted".to_owned(),
                    },
                    Err(e) => CliOutput {
                        status: Status::Err,
                        output: format!("DB Insertion Error: {}", e),
                    },
                }
            }
        },
        Err(c) => {
            // Error with the Quiery (Cursor not OK)
            CliOutput {
                status: Status::Err,
                output: format!("DB Quiery Error {}", c),
            }
        }
    }

    // -----------------------------------

    // let insert_result = collection.insert_one(&asset, None).await;
    // println!("create");
    // println!("collection: {:?}", collection);
    // println!("Asset: {:?}", asset);

    // -----------------------------------
    // CliOutput {
    //     status: Status::Ok,
    //     output: "asset created".to_owned(),
    // }
}
pub fn update(collection: mongodb::Collection<Asset>, asset: Asset) -> CliOutput {
    // > required:
    // asset_name, location, source
    // OR
    // asset_id

    println!("update asset");

    // get latest version and increment
    // create new Version struct and push to Vec > add to collection

    println!("collection: {:?}", collection);
    println!("Asset: {:?}", asset);

    CliOutput {
        status: Status::Ok,
        output: "asset updated".to_owned(),
    }
}
pub fn get_source(collection: mongodb::Collection<Asset>, asset: Asset) -> CliOutput {
    // > required:
    // datapath

    println!("get source");

    // parse datapath and extract asset_name, location, version ?
    // quiery and return source.

    println!("collection: {:?}", collection);
    println!("Asset: {:?}", asset);

    CliOutput {
        status: Status::Ok,
        output: "source file: xxxx".to_owned(),
    }
}

pub fn delete(collection: mongodb::Collection<Asset>, asset: Asset) -> CliOutput {
    // > required:
    // asset_name, location, source, version
    // OR
    // asset_id, version

    println!("mark asset for deletion");

    // find asset and update status to "purge"
    // status should be an Enum: online/purge/deleted

    println!("collection: {:?}", collection);
    println!("Asset: {:?}", asset);
    CliOutput {
        status: Status::Ok,
        output: "asset marked for deletion".to_owned(),
    }
}

pub fn get_latest(collection: mongodb::Collection<Asset>, asset: Asset) -> CliOutput {
    // > required:
    // asset_name, location, source
    // OR
    // asset_id

    println!("get latest verions");
    println!("collection: {:?}", collection);
    println!("Asset: {:?}", asset);
    CliOutput {
        status: Status::Ok,
        output: "latest version is xxx".to_owned(),
    }
}
