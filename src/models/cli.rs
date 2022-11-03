use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "TodoApp")]
#[command(author = "Panos Foli")]
#[command(version = "1.0")]
#[command(about = "Using Clap to manage a todo list", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Submit a todo entry
    New,
    /// Read all the entries
    Ls,
    /// Edit a specific entry
    Edit,
    /// Delete a specific entry
    Delete,
    /// Update the status of an entry
    Status,
}
