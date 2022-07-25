use crate::errors::{CliOutput, Status};
use crate::parse_args::Asset;
use crate::parse_args::JsonString;

// pub use crate::assetdef::Asset;
use crate::assetdef::AssetStatus;
pub use crate::assetdef::AssetVersion;

// CRUD functions

// use bson::{bson, Bson};
use mongodb::{bson::doc, Client, Collection};

pub async fn create(collection: mongodb::Collection<Asset>, json: JsonString) -> CliOutput {
    // > required:
    // asset_name, location, source

    let first_version = AssetVersion {
        version: 1 as u32,
        datapath: json.datapath.unwrap(),
        source: json.source.unwrap(),
        approved: false,
        status: AssetStatus::Online,
    };

    let mut versions: Vec<AssetVersion> = Vec::new();
    versions.push(first_version);

    let asset = Asset {
        name: json.name.as_ref().unwrap().to_string(),
        location: json.location.unwrap(),
        versions,
    };

    // find doc from name and location
    let cursor = collection
        .find_one(Some(doc! { "name": &json.name }), None)
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

// ------------------- FIND BY ID --------------------------------
// this should be done in utils.rs (we are just parsing arguments here)

// let objid = ObjectId::parse_str(&asset.id.unwrap());
// let objid_: ObjectId;
// if objid.is_ok() {
//     // let cursor = coll.find_one(Some(doc! { "_id": &objid.unwrap() }), None).await;
//     let cursor = coll.find_one(Some(doc! { "_id": &objid.Ok() }), None).await;
// }
// ---------------------------------------------------------------
// ## ## ## ## ## ##
// ## ## ## ## ## ##
pub async fn update(collection: mongodb::Collection<Asset>, json: JsonString) -> CliOutput {
    // > required:
    // asset_name, location, source
    // OR
    // asset_id

    let cursor = collection
        .find_one(
            Some(doc! { "name": &json.name , "location": &json.location}),
            None,
        )
        .await;

    match cursor {
        Ok(c) => match &c {
            Some(c) => {
                let last_version = c.versions.get(c.versions.len() - 1);

                let new_version: u32 = last_version.unwrap().version + 1;
                // TO DO: stop being lazy and remove "unwrap"

                let next_asset_version = AssetVersion {
                    version: new_version,
                    datapath: json.datapath.unwrap(),
                    source: json.source.unwrap(),
                    approved: false,
                    status: AssetStatus::Online,
                };

                let db_update_result = collection
                    .update_one(
                        doc! { "name": &json.name , "location":&json.location},
                        doc! { "$push": { "versions": &next_asset_version } },
                        None,
                    )
                    .await;

                CliOutput {
                    status: Status::Ok,
                    output: "Asset found in DB".to_owned(),
                }
            }
            None => CliOutput {
                status: Status::Err,
                output: format!("Asset not found in DB "),
            },
        },
        Err(c) => CliOutput {
            status: Status::Err,
            output: format!("DB Quiery Error {}", c),
        },
    }
}
pub async fn get_source(collection: mongodb::Collection<Asset>, json: JsonString) -> CliOutput {
    println!("get source");
    println!("....... {:?}", json);

    let cursor = collection
        .find_one(
            Some(doc! { "name": &json.name.unwrap() , "location": &json.location.unwrap()}),
            None,
        )
        .await;

    match cursor {
        Ok(c) => match &c {
            Some(c) => {
                for asset_version in &c.versions {
                    if asset_version.version == json.version.unwrap() {
                        let source = &asset_version.source;
                        return CliOutput {
                            status: Status::Ok,
                            output: source.to_owned(),
                        };
                    }
                }
                return CliOutput {
                    status: Status::Ok,
                    output: "Asset version not found".to_owned(),
                };
            }
            None => CliOutput {
                status: Status::Ok,
                output: "Asset NOT found in DB!".to_owned(),
            },
        },

        Err(c) => CliOutput {
            status: Status::Err,
            output: format!("DB Quiery Error: {}", c),
        },
    }

    // CliOutput {
    //     status: Status::Ok,
    //     output: "source file: xxxx".to_owned(),
    // }
}

pub async fn delete(collection: mongodb::Collection<Asset>, args: JsonString) -> CliOutput {
    // > required:
    // asset_name, location, source, version
    // OR
    // asset_id, version

    println!("mark asset for deletion");

    // find asset and update status to "purge"
    // status should be an Enum: online/purge/deleted

    println!("collection: {:?}", collection);
    println!("Asset: {:?}", args);
    CliOutput {
        status: Status::Ok,
        output: "asset marked for deletion".to_owned(),
    }
}

pub async fn get_latest(collection: mongodb::Collection<Asset>, args: JsonString) -> CliOutput {
    // > required:
    // asset_name, location, source
    // OR
    // asset_id

    println!("get latest verions");
    println!("collection: {:?}", collection);
    println!("Asset: {:?}", args);
    CliOutput {
        status: Status::Ok,
        output: "latest version is xxx".to_owned(),
    }
}
