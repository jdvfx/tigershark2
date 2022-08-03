use crate::assetdef::{AssetStatus, AssetVersion};
use crate::errors::CliOutput;
use crate::parse_args::{Asset, AssetJson};
use mongodb::bson::doc;

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
        .find_one(Some(doc! { "name": &asset.name }), None)
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
    let cursor = collection
        .find_one(
            Some(doc! { "name": &json.name , "location": &json.location}),
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
    let cursor = collection
        .find_one(
            Some(doc! { "name": &json.name, "location": &json.location}),
            None,
        )
        .await;

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

pub async fn delete(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    let db_delete_result = collection
    .update_one(
        doc! { "name": &json.name, "location": &json.location, "versions.version":&json.version},
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
    let cursor = collection
        .find_one(
            Some(doc! { "name": &json.name , "location": &json.location}),
            None,
        )
        .await;

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

// ------------------- FIND BY ID --------------------------------
// this should be done in utils.rs (we are just parsing arguments here)

// let objid = ObjectId::parse_str(&asset.id.unwrap());
// let objid_: ObjectId;
// if objid.is_ok() {
//     // let cursor = coll.find_one(Some(doc! { "_id": &objid.unwrap() }), None).await;
//     let cursor = coll.find_one(Some(doc! { "_id": &objid.Ok() }), None).await;
// }
// ---------------------------------------------------------------
