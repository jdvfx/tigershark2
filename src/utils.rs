use crate::assetdef::{AssetStatus, AssetVersion};
use crate::errors::CliOutput;
use crate::parse_args::{Asset, JsonString};
use mongodb::bson::doc;

// CRUD functions
pub async fn create(collection: mongodb::Collection<Asset>, json: JsonString) -> CliOutput {
    let first_version = AssetVersion {
        version: 1_u32,
        datapath: json.datapath.unwrap(),
        source: json.source.unwrap(),
        approved: false,
        status: AssetStatus::Online,
    };

    let versions: Vec<AssetVersion> = vec![first_version];

    let asset = Asset {
        name: json.name.as_ref().unwrap().to_string(),
        location: json.location.unwrap(),
        versions,
    };

    let cursor = collection
        .find_one(Some(doc! { "name": &json.name }), None)
        .await;

    match cursor {
        Ok(c) => match &c {
            Some(c) => CliOutput::new("ok", "Asset already exists"),
            None => {
                // asset not found, then insert it
                let insert_result = collection.insert_one(&asset, None).await;
                match insert_result {
                    Ok(..) => CliOutput::new("ok", "Asset inserted"),
                    Err(e) => CliOutput::new("err", &format!("DB Insertion Error:  {}", e)),
                }
            }
        },
        Err(c) => CliOutput::new("err", &format!("DB Quiery Error {}", c)),
    }
}

pub async fn update(collection: mongodb::Collection<Asset>, json: JsonString) -> CliOutput {
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
                    datapath: json.datapath.unwrap(),
                    source: json.source.unwrap(),
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
                // TODO: handle the result, could fail....
                CliOutput::new("ok", "New version inserted")
            }
            None => CliOutput::new("err", "Asset not found in DB"),
        },
        Err(c) => CliOutput::new("err", &format!("DB Quiery Error {}", c)),
    }
}
pub async fn get_source(collection: mongodb::Collection<Asset>, json: JsonString) -> CliOutput {
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

pub async fn delete(collection: mongodb::Collection<Asset>, json: JsonString) -> CliOutput {
    let db_delete_result = collection
    .update_one(
        doc! { "name": &json.name, "location": &json.location, "versions.version":&json.version},
        doc! { "$set": { "versions.$.status": "Purge" } },
        None,
    )
    .await;

    println!(">>>{:?}<<<", &db_delete_result);

    // db_delete_result.unwrap().modified_count
    // db_delete_result.unwrap().matched_count

    // match o.modified_count {
    // 0 => CliOutput::new("err", "Delete failed"),
    // _ => CliOutput::new(
    //     "ok",
    //     &format!("{:?} v{:?} marked for purge", &json.name, &json.version),
    // ),

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

pub async fn get_latest(collection: mongodb::Collection<Asset>, json: JsonString) -> CliOutput {
    let cursor = collection
        .find_one(
            Some(doc! { "name": &json.name , "location": &json.location}),
            None,
        )
        .await;

    match cursor {
        Ok(c) => match &c {
            Some(c) => {
                let last_asset_version = c.versions.last();
                let last_version: u32 = last_asset_version.unwrap().version;
                // TO DO: stop being lazy and remove "unwrap"
                CliOutput::new("ok", &last_version.to_string())
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
