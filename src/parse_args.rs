use clap::Parser;
use serde::{Deserialize, Serialize};

pub use crate::assetdef::{Asset, AssetStatus, AssetVersion};
use crate::errors::CliOutput;

#[derive(Debug)]
pub enum CommandType {
    Create,
    Update,
    Source,
    Delete,
    Latest,
    Approve,
}

#[derive(Debug)]
pub struct Command {
    pub command: CommandType,
    pub json: AssetJson,
}

/// Parse Command and Asset(json) arguments
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
    pub depend: Option<Vec<String>>,
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
    pub depend: Vec<String>,
    pub id: String,
}
// create default empty values if missing
// removes the need for unwrap() when executing CRUD commands
fn json_unwrap_or(json_o: JsonOption) -> AssetJson {
    AssetJson {
        name: json_o.name.unwrap_or_else(|| "".to_owned()),
        location: json_o.location.unwrap_or_else(|| "".to_owned()),
        source: json_o.source.unwrap_or_else(|| "".to_owned()),
        datapath: json_o.datapath.unwrap_or_else(|| "".to_owned()),
        version: json_o.version.unwrap_or(0),
        depend: json_o.depend.unwrap_or_else(|| Vec::<String>::new()),
        id: json_o.id.unwrap_or_else(|| "".to_owned()),
    }
}

// pub fn get_args() -> Option<Command> {
pub fn get_args() -> Result<Command, CliOutput> {
    //
    let args = Args::parse();
    // >>> ASSET ---
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
    // to check if json values are present for the current command
    let a_name = asset.name.is_some();
    let a_location = asset.location.is_some();
    let a_source = asset.source.is_some();
    let a_datapath = asset.datapath.is_some();
    let a_version = asset.version.is_some();
    // let a_depend = asset.depend.is_some();
    let a_id = asset.id.is_some();

    // unpack JsonOption into JsonString
    let asset_unwrapped: AssetJson = json_unwrap_or(asset);

    // >>> COMMAND <<<
    // for each command, checks that the correct json values are present
    match args.command.as_str() {
        "create" => match a_name && a_location && a_source && a_datapath {
            true => Ok(Command {
                command: CommandType::Create,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "latest : Asset missing some Keys")),
        },
        "update" => match (a_name && a_location || a_id) && a_source && a_datapath {
            true => Ok(Command {
                command: CommandType::Update,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "latest : Asset missing some Keys")),
        },
        "source" => match (a_name && a_location || a_id) && a_version {
            true => Ok(Command {
                command: CommandType::Source,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "latest : Asset missing some Keys")),
        },
        "delete" => match (a_name && a_location || a_id) && a_version {
            true => Ok(Command {
                command: CommandType::Delete,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "latest : Asset missing some Keys")),
        },
        "latest" => match a_name && a_location || a_id {
            true => Ok(Command {
                command: CommandType::Latest,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "latest : Asset missing some Keys")),
        },
        "approve" => match (a_name && a_location || a_id) && a_version {
            true => Ok(Command {
                command: CommandType::Approve,
                json: asset_unwrapped,
            }),
            _ => Err(CliOutput::new("err", "latest : Asset missing some Keys")),
        },
        _ => Err(CliOutput::new("err", "invalid a command")),
    }
}
