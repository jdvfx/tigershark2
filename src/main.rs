#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

pub mod parse_args;
use parse_args::CommandType;

pub mod db;
pub mod errors;
pub mod utils;

fn main() {
    // parse args
    let command = parse_args::get_args();
    match command {
        Some(c) => {
            // Connect to DB
            let collection = db::connect_to_db();
            println!("{collection}");
            //
            // Execute one of the CRUD commands
            let asset = c.asset;
            let output = match c.command {
                CommandType::Create => utils::create(collection, asset),
                CommandType::Update => utils::update(collection, asset),
                CommandType::GetSource => utils::get_source(collection, asset),
                CommandType::Delete => utils::delete(collection, asset),
                CommandType::GetLatest => utils::get_latest(collection, asset),
            };
            //
            // exit nicely, or panic
            errors::exit(output);
        }
        None => println!("no command"),
    }
}
