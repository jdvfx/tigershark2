use clap::Parser;
use serde::{Deserialize, Serialize};

pub use crate::assetdef::Asset;
pub use crate::assetdef::AssetVersion;
pub use crate::assetdef::Status;

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
    pub asset: Asset,
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

pub fn get_args() -> Option<Command> {
    let args = Args::parse();

    // --- ASSET ---
    // Asset is defined in assetdef.rs
    // get asset String from args and try to parse using struct above
    let asset_str = args.asset.to_string();
    let asset_result: serde_json::Result<JsonString> = serde_json::from_str(&asset_str);
    let asset: JsonString = match asset_result {
        Ok(a) => a,
        Err(r) => {
            print!("Err: bad json format: {} : {:?}", asset_str, r);
            panic!();
        }
    };
    let a_name = asset.name.is_some();
    let a_location = asset.location.is_some();
    let a_source = asset.source.is_some();
    let a_datapath = asset.datapath.is_some();
    let a_version = asset.version.is_some();

    // --- COMMAND ---
    let c = args.command;
    let cc: &str = &c;
    match cc {
        "create" => {
            if a_name && a_location && a_source && a_datapath {
                println!(">create : Asset Good");

                let first_version = AssetVersion {
                    version: 1 as u32,
                    location: asset.location.unwrap(),
                    datapath: asset.datapath.unwrap(),
                    source: asset.source.unwrap(),
                    approved: false,
                    status: Status::Online,
                };

                let asset = Asset {
                    name: asset.name.unwrap(),
                    version: first_version,
                };
                let command = CommandType::Create;
                return Some(Command { command, asset });
            } else {
                println!("create : Asset missing some Keys");
            }
        }
        "update" => {
            println!(">update");
        }
        "source" => {
            println!(">source");
        }
        "delete" => {
            println!(">delete");
            // can take ((name&&location)||ID)&&version
        }
        "latest" => {
            println!(">latest");
        }
        _ => {
            println!("NOT a command");
        }
    }

    //     approved: false,
    //     status: Status::Online,
    //

    let status = Status::Online;
    let asset_version = AssetVersion {
        version: 1 as u32,
        location: "location".to_owned(),
        datapath: "datapath".to_owned(),
        source: "source".to_owned(),
        approved: false,
        status: Status::Online,
    };

    let asset = Asset {
        name: "asset_name".to_owned(),
        version: asset_version,
    };
    // let version = AssetVersion {
    //     version: 1 as u32,
    //     source: "source_path".to_owned(),

    // };

    // let asset = Asset {
    //     name: "my_asset".to_owned(),
    //     location: "show_seq_shot".to_owned(),
    //     source: "source_file.hip".to_owned(),
    //     datapath: "/data/path/file/test.bgeo.sc".to_owned(),
    //     version,
    // };

    let command = CommandType::Create;
    Some(Command { command, asset })
}
