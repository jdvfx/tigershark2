use crate::assetdef::{AssetStatus, AssetVersion};
use crate::errors::CliOutput;
use crate::parse_args::{Asset, AssetJson};
use mongodb::bson::{doc, oid::ObjectId};

// CRUD functions
pub async fn create(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    let first_version = AssetVersion {
        version: 1_u32,
        datapath: json.datapath,
        source: json.source,
        approved: false,
        status: AssetStatus::Online,
    };

    let versions: Vec<AssetVersion> = vec![first_version];

    let asset = Asset {
        name: json.name,
        location: json.location,
        versions,
    };

    let cursor = collection
        .find_one(doc! { "name": &asset.name }, None)
        .await;

    match cursor {
        Ok(c) => match &c {
            Some(..) => CliOutput::new("ok", "Asset already exists"),
            None => {
                // asset not found, then insert it
                let insert_result = collection.insert_one(&asset, None).await;
                match insert_result {
                    Ok(..) => CliOutput::new("ok", "Asset inserted"),
                    Err(e) => CliOutput::new("err", &format!("DB Insertion Error:  {}", e)),
                }
            }
        },
        Err(e) => CliOutput::new("err", &format!("DB Quiery Error {}", e)),
    }
}

pub async fn update(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    // a_id && a_source && a_datapath
    //
    let cursor = collection
        .find_one(
            doc! { "name": &json.name , "location": &json.location},
            None,
        )
        .await;

    match cursor {
        Ok(c) => match &c {
            Some(c) => {
                let new_version: u32 = match c.versions.last() {
                    Some(v) => v.version + 1,
                    None => return CliOutput::new("err", "No Asset version found"),
                };

                let next_asset_version = AssetVersion {
                    version: new_version,
                    datapath: json.datapath,
                    source: json.source,
                    approved: false,
                    status: AssetStatus::Online,
                };

                // TO DO: check if another version has the same datapath
                // if so, that asset doesn't need to be inserted
                // should return an Error

                // push a new AssetVersion into versions vector
                let db_update_result = collection
                    .update_one(
                        doc! { "name": &json.name , "location":&json.location},
                        doc! { "$push": { "versions": &next_asset_version } },
                        None,
                    )
                    .await;
                match db_update_result {
                    Ok(..) => CliOutput::new("ok", "New version inserted"),
                    Err(e) => CliOutput::new("err", &format!("Error: {:?}", e)),
                }
                // TODO: handle the result, could fail....
            }
            None => CliOutput::new("err", "Asset not found in DB"),
        },
        Err(c) => CliOutput::new("err", &format!("DB Quiery Error {}", c)),
    }
}
pub async fn get_source(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    let filter: bson::Document;
    if json.id != "" {
        let objid = ObjectId::parse_str(json.id.to_string());
        match objid {
            Ok(o) => filter = doc! {"_id": o},
            Err(e) => return CliOutput::new("err", &format!("ID not found: {:?}", e)),
        }
    } else {
        filter = doc! { "name": &json.name , "location": &json.location};
    }

    let cursor = collection.find_one(filter, None).await;

    match cursor {
        Ok(c) => match &c {
            Some(c) => {
                for asset_version in &c.versions {
                    if asset_version.version == json.version {
                        let source = &asset_version.source;
                        return CliOutput::new("ok", &source.to_owned());
                    }
                }
                CliOutput::new("ok", "Asset version not found")
            }
            None => CliOutput::new("ok", "Asset not found in DB"),
        },
        Err(c) => CliOutput::new("err", &format!("DB Quiery Error: {}", c)),
    }
}

// fn get_id(json:AssetJson) -> ObjectId{

// }

pub async fn delete(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    let filter: bson::Document;
    if json.id != "" {
        let objid = ObjectId::parse_str(json.id.to_string());
        match objid {
            Ok(o) => filter = doc! {"_id": o,"versions.version":&json.version},
            Err(e) => return CliOutput::new("err", &format!("ID not found: {:?}", e)),
        }
    } else {
        filter = doc! { "name": &json.name, "location": &json.location, "versions.version":&json.version};
    }

    let db_delete_result = collection
        .update_one(
            filter,
            doc! { "$set": { "versions.$.status": "Purge" } },
            None,
        )
        .await;

    match db_delete_result {
        Ok(o) => {
            if o.matched_count == 0 {
                return CliOutput::new("err", "Delete failed: asset version not found");
            }

            match o.modified_count {
                0 => CliOutput::new("ok", "Already tagged for deletion"),
                _ => CliOutput::new("ok", "Tagged for Deletion"),
            }
        }
        Err(e) => CliOutput::new("err", &format!("Delete failed:{:?}", e)),
    }
}

pub async fn get_latest(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    let filter: bson::Document;
    if json.id != "" {
        let objid = ObjectId::parse_str(json.id.to_string());
        match objid {
            Ok(o) => filter = doc! {"_id": o},
            Err(e) => return CliOutput::new("err", &format!("ID not found: {:?}", e)),
        }
    } else {
        filter = doc! { "name": &json.name , "location": &json.location};
    }

    let cursor = collection.find_one(filter, None).await;

    match cursor {
        Ok(c) => match &c {
            Some(c) => {
                let last_version: u32 = match c.versions.last() {
                    Some(v) => v.version,
                    None => return CliOutput::new("err", "No version found"),
                };
                CliOutput::new("ok", &format!("{}", last_version))
            }
            None => CliOutput::new("err", "Asset not found in DB"),
        },
        Err(c) => CliOutput::new("err", &format!("DB Quiery Error {}", c)),
    }
}
