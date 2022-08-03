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
    let command = parse_args::get_args();
    match command {
        Ok(c) => {
            // Connect to DB
            let uri: String = "mongodb://localhost:27017".to_owned();
            let db_name: String = "sharks".to_owned();
            let coll_name: String = "tiger".to_owned();
            //
            let collection = db::connect_to_db(uri, db_name, coll_name);
            match collection.await {
                Some(collection) => {
                    let json = c.json;
                    // Execute one of the CRUD commands
                    cli_output = match c.command {
                        CommandType::Create => utils::create(collection, json).await,
                        CommandType::Update => utils::update(collection, json).await,
                        CommandType::GetSource => utils::get_source(collection, json).await,
                        CommandType::Delete => utils::delete(collection, json).await,
                        CommandType::GetLatest => utils::get_latest(collection, json).await,
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
