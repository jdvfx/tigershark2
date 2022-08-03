use clap::Parser;
use serde::{Deserialize, Serialize};

pub use crate::assetdef::{Asset, AssetStatus, AssetVersion};
use crate::errors::CliOutput;
// use mongodb::bson::oid::ObjectId;

#[derive(Debug)]
pub enum CommandType {
    Create,
    Update,
    GetSource,
    Delete,
    GetLatest,
}

// --create -c
// --update -u
// --source -s
// --latest -l
// --delete -d

#[derive(Debug)]
pub struct Command {
    pub command: CommandType,
    pub json: AssetJson,
}

/// CLI Asset tracker with MondoDB
#[derive(Parser, Debug)]
#[clap(author="Julien D.", version, about, long_about = None)]
struct Args {
    /// CRUD command
    #[clap(short, long, value_parser)]
    command: String,

    /// json string representing the asset
    #[clap(short, long, value_parser)]
    asset: String,
}

// serialized by Serde (could have missing fields: Options)
#[derive(Debug, Serialize, Deserialize)]
struct JsonOption {
    pub name: Option<String>,
    pub location: Option<String>,
    pub source: Option<String>,
    pub datapath: Option<String>,
    pub version: Option<u32>,
    pub id: Option<String>,
}
// the asset json that gets passed to the CRUD function
#[derive(Debug)]
pub struct AssetJson {
    pub name: String,
    pub location: String,
    pub source: String,
    pub datapath: String,
    pub version: u32,
    pub id: String,
}
//
fn json_unwrap_or(json_o: JsonOption) -> AssetJson {
    AssetJson {
        name: json_o.name.unwrap_or_else(|| "".to_owned()),
        location: json_o.location.unwrap_or_else(|| "".to_owned()),
        source: json_o.source.unwrap_or_else(|| "".to_owned()),
        datapath: json_o.datapath.unwrap_or_else(|| "".to_owned()),
        version: json_o.version.unwrap_or(0),
        id: json_o.id.unwrap_or_else(|| "".to_owned()),
    }
}

// pub fn get_args() -> Option<Command> {
pub fn get_args() -> Result<Command, CliOutput> {
    let args = Args::parse();

    // --- ASSET ---
    // Asset is defined in assetdef.rs
    // get asset String from args and try to parse using struct above
    let asset_str = args.asset.to_string();
    let asset_result: serde_json::Result<JsonOption> = serde_json::from_str(&asset_str);
    let asset: JsonOption = match asset_result {
        Ok(a) => a,
        Err(r) => {
            return Err(CliOutput::new(
                "err",
                &format!("Err: bad json format: {} : {:?}", asset_str, r),
            ))
        }
    };
    let a_name = asset.name.is_some();
    let a_location = asset.location.is_some();
    let a_source = asset.source.is_some();
    let a_datapath = asset.datapath.is_some();
    let a_version = asset.version.is_some();
    let a_id = asset.id.is_some();

    // unpack JsonOption into JsonString
    let asset_unwrapped: AssetJson = json_unwrap_or(asset);

    // --- COMMAND ---
    let c = args.command;

    // for each command, checks that the correct asset attributes are present
    let cc: &str = &c;
    match cc {
        "create" => {
            if a_name && a_location && a_source && a_datapath {
                let command = CommandType::Create;
                Ok(Command {
                    command,
                    json: asset_unwrapped,
                })
            } else {
                Err(CliOutput::new("err", "latest : Asset missing some Keys"))
            }
        }
        "update" => {
            if a_name && a_location && a_source && a_datapath || a_id && a_source && a_datapath {
                // if ID is used: handled in Utils
                let command = CommandType::Update;
                Ok(Command {
                    command,
                    json: asset_unwrapped,
                })
            } else {
                Err(CliOutput::new("err", "latest : Asset missing some Keys"))
            }
        }
        "source" => {
            if a_name && a_location && a_version || a_id && a_version {
                // todo : search by ID and version
                let command = CommandType::GetSource;
                Ok(Command {
                    command,
                    json: asset_unwrapped,
                })
            } else {
                Err(CliOutput::new("err", "latest : Asset missing some Keys"))
            }
        }
        "delete" => {
            if a_name && a_location && a_version || a_id && a_version {
                let command = CommandType::Delete;
                Ok(Command {
                    command,
                    json: asset_unwrapped,
                })
            } else {
                Err(CliOutput::new("err", "latest : Asset missing some Keys"))
            }
        }
        "latest" => {
            if a_name && a_location || a_id {
                let command = CommandType::GetLatest;
                Ok(Command {
                    command,
                    json: asset_unwrapped,
                })
            } else {
                Err(CliOutput::new("err", "latest : Asset missing some Keys"))
            }
        }
        _ => Err(CliOutput::new("err", "invalid a command")),
    }
}
