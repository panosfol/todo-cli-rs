pub mod db;
mod models;
pub mod schema;

use clap::Parser;
use db::establish_connection;
use models::{
    cli::{Cli, Commands},
    entry::Entry,
};
use std::io;

fn main() {
    let _connection = &mut establish_connection();
    let cli = Cli::parse();
    let mut entry = Entry::default();

    match cli.command {
        Some(Commands::Create) => {
            println!("Enter title of the entry:");
            match io::stdin().read_line(&mut entry.title) {
                Ok(_) => {}
                Err(error) => println!("error: {}", error),
            }
            println!("Enter description:");
            match io::stdin().read_line(&mut entry.description) {
                Ok(_) => {}
                Err(error) => println!("error: {}", error),
            }
            println!("{}", entry.title);
            println!("{}", entry.description);
        }
        Some(Commands::Delete) => println!("command: delete"),
        Some(Commands::Edit) => println!("command: edit"),
        Some(Commands::Update) => println!("command: update"),
        Some(Commands::Read) => println!("command: read"),
        None => {},
    };
}
