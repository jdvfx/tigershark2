// Asset structure definition

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AssetStatus {
    Online,
    Purge,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetVersion {
    pub version: u32,
    pub datapath: String,
    pub source: String,
    pub approved: bool,
    pub status: AssetStatus,
}

// this goes in the DB, no touchy!
#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub location: String,
    pub version: AssetVersion,
}
