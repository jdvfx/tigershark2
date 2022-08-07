pub mod parse_args;
use errors::{exit_or_panic, CliOutput};
use parse_args::CommandType;

mod assetdef;
pub mod db;
pub mod errors;
pub mod utils;

#[tokio::main]
async fn main() {
    let cli_output: CliOutput;
    // parse args
    let args = parse_args::get_args();
    match args {
        Ok(args) => {
            // Connect to DB
            let uri: &str = "mongodb://localhost:27017";
            let db_name: &str = "sharks";
            let collection_name: &str = "tiger";
            //
            let collection = db::connect_to_db(uri, db_name, collection_name);
            match collection.await {
                Some(coll) => {
                    let json = args.json;
                    // Execute one of the CRUD commands
                    cli_output = match args.command {
                        CommandType::Create => utils::create(coll, json).await,
                        CommandType::Update => utils::update(coll, json).await,
                        CommandType::GetSource => utils::get_source(coll, json).await,
                        CommandType::Delete => utils::delete(coll, json).await,
                        CommandType::GetLatest => utils::get_latest(coll, json).await,
                    };
                }
                None => {
                    cli_output = CliOutput::new("err", "Error with the collection");
                }
            }
        }
        Err(o) => cli_output = o,
    }
    //
    exit_or_panic(cli_output);
}
