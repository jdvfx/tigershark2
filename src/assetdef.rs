// Asset structure definition
use mongodb::bson::{doc, Bson};
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub location: String,
    pub versions: Vec<AssetVersion>,
}

// used for update_one()
// Rust needs to know how to convert AssetVersion to Bson before
// pushing it to the DB, needs a From<AssetVersion> Trait
//
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

impl From<AssetVersion> for Bson {
    fn from(a: AssetVersion) -> Bson {
        let d = doc! {
            "version":a.version,
            "datapath":a.datapath,
            "source":a.source,
            "approved":a.approved,
            "status":a.status,
        };
        Bson::Document(d)
    }
}
