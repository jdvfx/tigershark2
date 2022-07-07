// Asset structure definition

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Online,
    Purge,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetVersion {
    pub version: u32,
    pub source: String,
    pub approved: bool,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub location: String,
    pub source: String,
    pub datapath: String,
    pub version: AssetVersion,
}
