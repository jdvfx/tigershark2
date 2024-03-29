// Asset structure definition
use mongodb::bson::{doc, Bson};
use serde::{Deserialize, Serialize};

use crate::parse_args::AssetJson;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AssetStatus {
    Online,
    Purge,
    Delete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetVersion {
    pub version: u32,
    pub datapath: String,
    pub source: String,
    pub approved: bool,
    pub status: AssetStatus,
    // pub depend: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub location: String,
    pub versions: Vec<AssetVersion>,
}
// create first version of an asset when using the Create command
impl Asset {
    pub fn first_version(json: AssetJson) -> Self {
        //
        let first_version = AssetVersion {
            version: 1_u32,
            datapath: json.datapath,
            source: json.source,
            approved: false,
            status: AssetStatus::Online,
            // depend: json.depend,
        };

        let versions: Vec<AssetVersion> = vec![first_version];

        Asset {
            name: json.name,
            location: json.location,
            versions,
        }
    }
}

// used for update_one()
// converts AssetStatus to Bson String
impl From<AssetStatus> for Bson {
    fn from(a: AssetStatus) -> Bson {
        let b = match a {
            AssetStatus::Online => "Online".to_owned(),
            AssetStatus::Purge => "Purge".to_owned(),
            AssetStatus::Delete => "Delete".to_owned(),
        };
        Bson::String(b)
    }
}

// used for update_one()
// converts AssetVersion to Bson Document
impl From<AssetVersion> for Bson {
    fn from(a: AssetVersion) -> Bson {
        let d = doc! {
            "version":a.version,
            "datapath":a.datapath,
            "source":a.source,
            "approved":a.approved,
            "status":a.status,
            // "depend":a.depend,
        };
        Bson::Document(d)
    }
}
