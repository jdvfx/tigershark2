use serde::{Deserialize, Serialize};

enum Status {
    online,
    purge,
    delete,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetVersion {
    version: u32,
    source: String,
    approved: bool,
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    name: String,
    location: String,
    source: String,
    datapath: String,
    version: AssetVersion,
}

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

pub fn get_args() -> Option<Command> {
    // parse the args with Clap
    // check args for command and json passed.
    // each command has a set of required elements in the json

    let asset = Asset {
        name: "my_asset".to_owned(),
        location: "show_seq_shot".to_owned(),
        source: "source_file.hip".to_owned(),
        datapath: "/data/path/file/test.bgeo.sc".to_owned(),
    };

    let command = CommandType::Create;
    Some(Command { command, asset })

}
