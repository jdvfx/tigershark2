use clap::Parser;
use serde::{Deserialize, Serialize};

pub use crate::assetdef::{Asset, AssetStatus, AssetVersion};
use crate::errors::CliOutput;
use mongodb::bson::oid::ObjectId;

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
// --delete -d
// --latest -l

#[derive(Debug)]
pub struct Command {
    pub command: CommandType,
    pub json: JsonString,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonString {
    pub name: Option<String>,
    pub location: Option<String>,
    pub source: Option<String>,
    pub datapath: Option<String>,
    pub version: Option<u32>,
    pub id: Option<String>,
}

// pub fn get_args() -> Option<Command> {
pub fn get_args() -> Result<Command, CliOutput> {
    let args = Args::parse();

    // --- ASSET ---
    // Asset is defined in assetdef.rs
    // get asset String from args and try to parse using struct above
    let asset_str = args.asset.to_string();
    let asset_result: serde_json::Result<JsonString> = serde_json::from_str(&asset_str);
    let asset: JsonString = match asset_result {
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

    // --- COMMAND ---
    let c = args.command;

    let cc: &str = &c;
    match cc {
        "create" => {
            if a_name && a_location && a_source && a_datapath {
                let command = CommandType::Create;
                Ok(Command {
                    command,
                    json: asset,
                })
            } else {
                Err(CliOutput::new("err", "latest : Asset missing some Keys"))
            }
        }
        "update" => {
            if a_name && a_location && a_source && a_datapath || a_id && a_source && a_datapath {
                if a_id {
                    // TODO: NOT IMPLEMENTED YET
                    let command = CommandType::Update;
                    Ok(Command {
                        command,
                        json: asset,
                    })
                } else {
                    let command = CommandType::Update;
                    Ok(Command {
                        command,
                        json: asset,
                    })
                }
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
                    json: asset,
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
                    json: asset,
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
                    json: asset,
                })
            } else {
                Err(CliOutput::new("err", "latest : Asset missing some Keys"))
            }
        }
        _ => Err(CliOutput::new("err", "invalid a command")),
    }

    //return no command by default
    // CliOutput::new("err", "parse failed")
}
