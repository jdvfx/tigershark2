pub mod parse_args;
use errors::{exit_or_panic, CliOutput};
use parse_args::CommandType;

mod assetdef;
pub mod db;
pub mod errors;
pub mod utils;
use std::env;

/// Usage
/// # 1) Set Environment variables
/// export MONGODB_URI='mongodb://localhost:27017'
/// export MONGODB_DB='sharks'
/// export MONGODB_COLL='tiger'
/// # 2) Insert a new asset
/// ../target/debug/tigershark2 -c create -a '{
/// "name":"Box",
/// "location":"my_box_location",
/// "source":"source_that_created_box",
/// "datapath":"/my/data/path/mybox"}'
#[tokio::main]
async fn main() {
    let cli_output: CliOutput;
    // parse args
    let args = parse_args::get_args();
    match args {
        Ok(args) => {
            // Connect to DB
            let uri: &str =
                &env::var("MONGODB_URI").expect("MONGODB_URI environment var not set!");
            let db_name: &str =
                &env::var("MONGODB_DB").expect("MONGODB_DB environment var not set!");
            let collection_name: &str =
                &env::var("MONGODB_COLL").expect("MONGODB_COLL environment var not set!");
            //
            let collection = db::connect_to_db(uri, db_name, collection_name);
            match collection.await {
                Some(coll) => {
                    let json = args.json;
                    // Execute one of the CRUD commands
                    cli_output = match args.command {
                        CommandType::Create => utils::create(coll, json).await,
                        CommandType::Update => utils::update(coll, json).await,
                        CommandType::Source => utils::source(coll, json).await,
                        CommandType::Delete => utils::delete(coll, json).await,
                        CommandType::Latest => utils::latest(coll, json).await,
                    };
                }
                None => {
                    cli_output = CliOutput::new("err", "Error with the collection");
                }
            }
        }
        Err(o) => cli_output = CliOutput::new("err", &format!("Error parsing args: {:?}", o)),
    }
    exit_or_panic(cli_output);
}
