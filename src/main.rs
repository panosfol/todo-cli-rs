pub mod db;
mod models;
pub mod schema;

use clap::{Parser, Subcommand};
use db::{create_entry, delete_entry, edit_entry, establish_connection, get_entries, update_entry};
use dialoguer::{theme::ColorfulTheme, Select};
use models::entry::{EditedEntry, NewEntry};
use std::io;

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

fn main() {
	let connection = &mut establish_connection();
	let cli = Cli::parse();

	match cli.command {
		Some(Commands::New) => {
			let mut new_entry = NewEntry::default();
			println!("Enter title of the entry:");
			match io::stdin().read_line(&mut new_entry.title) {
				Ok(_) => {},
				Err(error) => println!("error: {}", error),
			}
			println!("Enter description:");
			match io::stdin().read_line(&mut new_entry.description) {
				Ok(_) => {},
				Err(error) => println!("error: {}", error),
			}
			new_entry.status = "Active".to_string();
			new_entry.title = new_entry.title.trim().to_string();
			new_entry.description = new_entry.description.trim().to_string();
			create_entry(connection, new_entry);
		},
		Some(Commands::Delete) => {
			let entries = get_entries(connection).unwrap();
			let entries_title: Vec<String> = entries.into_iter().map(|p| p.title).collect();
			let selection = Select::with_theme(&ColorfulTheme::default())
				.with_prompt("Pick an entry to delete")
				.items(&entries_title[..])
				.interact()
				.unwrap();

			delete_entry(connection, entries_title[selection].clone());
		},

		Some(Commands::Edit) => {
			let entries = get_entries(connection).unwrap();
			let mut edited_entry = EditedEntry::default();
			let entries_title: Vec<String> = entries.into_iter().map(|p| p.title).collect();
			let entry_fields = &["Title", "Description", "Both"];
			let selection_entries = Select::with_theme(&ColorfulTheme::default())
				.with_prompt("Pick an entry to edit")
				.items(&entries_title[..])
				.interact()
				.unwrap();
			let selection_edit = Select::with_theme(&ColorfulTheme::default())
				.with_prompt("Pick a field to edit")
				.items(&entry_fields[..])
				.interact()
				.unwrap();
			match selection_edit {
				0 => {
					println!("Give new title for chosen entry:");
					match io::stdin().read_line(&mut edited_entry.title) {
						Ok(_) => {},
						Err(error) => println!("error: {}", error),
					}
				},
				1 => {
					println!("Give new description for chosen entry:");
					match io::stdin().read_line(&mut edited_entry.description) {
						Ok(_) => {},
						Err(error) => println!("error: {}", error),
					}
				},
				2 => {
					println!("Give new title for chosen entry:");
					match io::stdin().read_line(&mut edited_entry.title) {
						Ok(_) => {},
						Err(error) => println!("error: {}", error),
					}
					println!("Give new description for chosen entry:");
					match io::stdin().read_line(&mut edited_entry.description) {
						Ok(_) => {},
						Err(error) => println!("error: {}", error),
					}
				},
				_ => (),
			}
			edited_entry.title = edited_entry.title.trim().to_string();
			edited_entry.description = edited_entry.description.trim().to_string();
			edit_entry(connection, entries_title[selection_entries].clone(), edited_entry)
		},

		Some(Commands::Status) => {
			let entries = get_entries(connection).unwrap();
			let entries_title: Vec<String> = entries.clone().into_iter().map(|p| p.title).collect();
			let entries_status = &["Abandoned", "Active", "Done"];
			let selection_entries = Select::with_theme(&ColorfulTheme::default())
				.with_prompt("Pick an entry to change status")
				.items(&entries_title[..])
				.interact()
				.unwrap();
			let selection_status = Select::with_theme(&ColorfulTheme::default())
				.with_prompt("Pick updated status")
				.items(&entries_status[..])
				.interact()
				.unwrap();
			update_entry(
				connection,
				entries_title[selection_entries].clone(),
				entries_status[selection_status].to_string(),
			);
		},

		Some(Commands::Ls) => {
			let entries = get_entries(connection);
			match entries {
				Ok(ref data) =>
					for i in data {
						println!("title: {}, status : {}", i.title, i.status);
					},
				Err(ref err) => {
					println!("{}", err);
				},
			}
			let entries_title: Vec<String> =
				entries.as_ref().unwrap().clone().into_iter().map(|p| p.title).collect();
			let selection_entries = Select::with_theme(&ColorfulTheme::default())
				.with_prompt("Pick an entry to read description")
				.items(&entries_title[..])
				.interact()
				.unwrap();
			println!("{}", entries.unwrap().clone()[selection_entries].description);
		},
		None => {},
	};
}
