#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use mongodb::{bson::doc, bson::Document};
use mongodb::{Client, Collection};

pub mod parse_args;
use errors::{exit_or_panic, CliOutput, Status};
use parse_args::CommandType;

pub mod db;
pub mod errors;
pub mod utils;

mod assetdef;

#[tokio::main]
async fn main() {
    // put most of this in lib.rs
    // user should not be exposed to all this

    // parse args
    let command = parse_args::get_args();

    println!("{:?}", command);
    println!(" . . . . ");
    // panic!();

    match command {
        Some(c) => {
            // Connect to DB
            let collection = db::connect_to_db();
            match collection.await {
                Some(collection) => {
                    println!("collection found");

                    // println!("{:?}", collection);
                    // panic!();

                    let args = c.args;
                    // Execute one of the CRUD commands
                    let cli_output = match c.command {
                        CommandType::Create => utils::create(collection, args).await,
                        CommandType::Update => utils::update(collection, args),
                        CommandType::GetSource => utils::get_source(collection, args),
                        CommandType::Delete => utils::delete(collection, args),
                        CommandType::GetLatest => utils::get_latest(collection, args),
                    };
                    exit_or_panic(cli_output)
                }
                None => {
                    println!("Error with the collection");
                }
            }

            let cli_output = CliOutput {
                status: Status::Ok,
                output: "exit nicely".to_owned(),
            };
            exit_or_panic(cli_output);
        }
        None => println!("no command"),
    }
}
