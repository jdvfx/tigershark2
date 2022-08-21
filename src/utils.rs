#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use crate::assetdef::{AssetStatus, AssetVersion};
use crate::errors::CliOutput;
use crate::parse_args::{Asset, AssetJson};
use mongodb::bson::{doc, oid::ObjectId};

/// overrides search filter when using ID
fn filter_by_id(json: &AssetJson, filter: &mut bson::Document) {
    if !json.id.is_empty() {
        if let Ok(id) = ObjectId::parse_str(&json.id) {
            if json.version != 0 {
                *filter = doc! {"_id": id,"versions.version":&json.version}
            } else {
                *filter = doc! {"_id": id}
            }
        }
    }
}

/// Inserts asset into DB
/// # Required Json Asset fields
/// * `name` : the name of the asset
/// * `location` : asset location, typically: show/seq/shot
/// * `source` : the file that created the asset
/// * `datapath` : where the asset data is stored on disk
pub async fn create(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    //
    let asset = Asset::first_version(json);

    let cursor = collection
        .find_one(doc! { "name": &asset.name }, None)
        .await;

    match cursor {
        Ok(c) => match &c {
            Some(..) => CliOutput::new("err", "Asset already exists"),
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

/// Insert new version of existing asset
/// # Required Json Asset fields
/// * `name` : the name of the asset
/// * `location` : asset location, typically: show/seq/shot
/// * `id` : DB id can be used instead of name+location
/// * `source` : the file that created the asset
/// * `datapath` : where the asset data is stored on disk
pub async fn update(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    //
    let mut filter: bson::Document = doc! { "name": &json.name , "location": &json.location};
    filter_by_id(&json, &mut filter);

    let cursor = collection.find_one(filter.clone(), None).await;

    match cursor {
        Ok(c) => match &c {
            Some(c) => {
                // increment version number
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
                    depend: Vec::new(),
                };

                // push a new AssetVersion into versions vector
                let db_update_result = collection
                    .update_one(
                        filter,
                        doc! { "$push": { "versions": &next_asset_version } },
                        None,
                    )
                    .await;
                match db_update_result {
                    Ok(..) => CliOutput::new("ok", "New version inserted"),
                    Err(e) => CliOutput::new("err", &format!("Error: {:?}", e)),
                }
            }
            None => CliOutput::new("err", "Asset not found in DB"),
        },
        Err(e) => CliOutput::new("err", &format!("DB Quiery Error {}", e)),
    }
}
/// Get the source file that created the asset
/// # Required Json Asset fields
/// * `name` : the name of the asset
/// * `location` : asset location, typically: show/seq/shot
/// * `id` : DB id can be used instead of name+location
pub async fn source(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    //
    let mut filter: bson::Document = doc! { "name": &json.name , "location": &json.location};
    filter_by_id(&json, &mut filter);

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
                CliOutput::new("err", "Asset version not found")
            }
            None => CliOutput::new("err", "Asset not found in DB"),
        },
        Err(e) => CliOutput::new("err", &format!("DB Quiery Error: {}", e)),
    }
}

/// Marks an asset version as "Purge"
/// # Required Json Asset fields
/// * `name` : the name of the asset
/// * `location` : asset location, typically: show/seq/shot
/// * `id` : DB id can be used instead of name+location
/// * `version` : Asset version
pub async fn delete(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    //
    let mut filter: bson::Document =
        doc! { "name": &json.name, "location": &json.location, "versions.version":&json.version};
    filter_by_id(&json, &mut filter);

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

/// Gets the latest version of an asset
/// # Required Json Asset fields
/// * `name` : the name of the asset
/// * `location` : asset location, typically: show/seq/shot
/// * `id` : DB id can be used instead of name+location
pub async fn latest(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    //
    let mut filter: bson::Document = doc! { "name": &json.name , "location": &json.location};
    filter_by_id(&json, &mut filter);

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
        Err(e) => CliOutput::new("err", &format!("DB Quiery Error {}", e)),
    }
}
/// Marks an asset version as "Approved"
/// # Required Json Asset fields
/// * `name` : the name of the asset
/// * `location` : asset location, typically: show/seq/shot
/// * `id` : DB id can be used instead of name+location
/// * `version` : Asset version
pub async fn approve(collection: mongodb::Collection<Asset>, json: AssetJson) -> CliOutput {
    //
    let mut filter: bson::Document =
        doc! { "name": &json.name, "location": &json.location, "versions.version":&json.version};
    filter_by_id(&json, &mut filter);

    let cursor = collection.find_one(filter.clone(), None).await;

    let x = &json.version.clone();

    match cursor {
        Ok(c) => match &c {
            Some(c) => {

                // I "should be able" to use get() directly but that doesn't work
                for i in &c.versions {
                    if &i.version == &json.version {
                        println!(">>>> {:?}, {:?}", &i.version, &i.depend);
                    }
                }

                // let aa = c.versions.get(v).unwrap();

                // let last_version: u32 = match c.versions.last() {
                //     Some(v) => v.version,
                //
                // let version = &json.version - 1;
                // let v = version.clone() as i32;

                // let vv = *x as u32;
                // TODO : replace the 0 with the current asset version passed in the JSON
                // let depend = &c.versions.get(vv).unwrap();
                let depend = 0;
                CliOutput::new("ok", &format!(">depend:{:?}", depend))
            }
            None => CliOutput::new("err", "Asset not found in DB"),
        },
        Err(e) => CliOutput::new("err", &format!("DB Quiery Error {}", e)),
    }

    // ---------------------------
    // TODO: find all the dependencies and approve them all
    // ---------------------------

    // let db_approve_result = collection
    //     .update_one(
    //         filter,
    //         doc! { "$set": { "versions.$.approved": true } },
    //         None,
    //     )
    //     .await;

    // match db_approve_result {
    //     Ok(o) => {
    //         if o.matched_count == 0 {
    //             return CliOutput::new("err", "Approval failed: asset version not found");
    //         }

    //         match o.modified_count {
    //             0 => CliOutput::new("ok", "Already approved"),
    //             _ => CliOutput::new("ok", "Version approved"),
    //         }
    //     }
    //     Err(e) => CliOutput::new("err", &format!("Approval failed:{:?}", e)),
    // }
}
