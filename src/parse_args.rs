#[derive(Debug)]
pub struct Asset {
    name: String,
    location: String,
    source: String,
    datapath: String,
}

#[derive(Debug)]
pub enum CommandType {
    Create,
    Update,
    GetSource,
    Delete,
    GetLatest,
}

#[derive(Debug)]
pub struct Command {
    pub command: CommandType,
    pub asset: Asset,
}

pub fn get_args() -> Option<Command> {
    let asset = Asset {
        name: "my_asset".to_owned(),
        location: "show_seq_shot".to_owned(),
        source: "source_file.hip".to_owned(),
        datapath: "/data/path/file/test.bgeo.sc".to_owned(),
    };

    let command = CommandType::Create;

    Some(Command { command, asset })
}
