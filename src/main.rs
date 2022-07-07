#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

pub mod parse_args;
use errors::{exit_or_panic, CliOutput, Status};
use parse_args::CommandType;

pub mod db;
pub mod errors;
pub mod utils;

fn main() {
    // put most of this in lib.rs
    // user should not be exposed to all this

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
            // exit nicely for now:
            let cli_output = CliOutput {
                status: Status::Ok,
                output: "exit nicely".to_owned(),
            };
            exit_or_panic(cli_output);
        }
        None => println!("no command"),
    }
}
