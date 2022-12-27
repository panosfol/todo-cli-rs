mod db;
mod models;
mod schema;

use clap::{Args, Parser, Subcommand};
use db::{create_entry, delete_entry, edit_entry, establish_connection, get_entries, update_entry};
use dialoguer::{theme::ColorfulTheme, Select};
use models::entry::{EditedEntry, NewEntry};
use std::io;

#[derive(Parser)]
#[command(name = "TodoApp")]
#[command(author = "Panos Foli")]
#[command(version = "1.0")]
#[command(about = "Using Clap to manage a todo list")]
#[command(propagate_version = true)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	/// Submit a todo entry
	New(EntryType),
	/// Read all the entries
	#[clap(alias = "list")]
	Ls(EntryType),
	/// Edit a specific entry
	Edit,
	/// Delete a specific entry
	#[clap(alias = "del")]
	Delete,
	/// Update the status of an entry
	Status,
}

#[derive(Args, Debug)]
pub struct EntryType {
	entry_type: Option<String>,
}

fn main() {
	let connection = &mut establish_connection();
	let cli = Cli::parse();

	match cli.command {
		Some(Commands::New(lol)) => {
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
				.interact();

			match selection {
				Ok(selected) => {
					delete_entry(connection, entries_title[selected].clone());
				},
				Err(error) => {
					println!("error: {}", error)
				},
			}
		},

		Some(Commands::Edit) => {
			let entries = get_entries(connection).unwrap();
			let mut edited_entry = EditedEntry::default();
			let entries_title: Vec<String> = entries.clone().into_iter().map(|p| p.title).collect();
			let selectable_fields = &["Title", "Description", "Both"];
			let selection_entries = Select::with_theme(&ColorfulTheme::default())
				.with_prompt("Pick an entry to edit")
				.items(&entries_title[..])
				.interact();
			match selection_entries {
				Ok(selected) => {
					let selected_title = &entries[selected].title;
					let selected_desc = &entries[selected].description;
					let selection_edit = Select::with_theme(&ColorfulTheme::default())
						.with_prompt("Pick a field to edit")
						.items(&selectable_fields[..])
						.interact()
						.unwrap();

					match selection_edit {
						0 => {
							println!("Give new title for chosen entry:");
							match io::stdin().read_line(&mut edited_entry.title) {
								Ok(_) => {
									edited_entry.title = edited_entry.title.trim().to_string();
									edited_entry.description = selected_desc.to_string();
									edit_entry(
										connection,
										entries_title[selected].clone(),
										edited_entry,
									);
								},
								Err(error) => println!("error: {}", error),
							}
						},
						1 => {
							println!("Give new description for chosen entry:");
							match io::stdin().read_line(&mut edited_entry.description) {
								Ok(_) => {
									edited_entry.description =
										edited_entry.description.trim().to_string();
									edited_entry.title = selected_title.to_string();
									edit_entry(
										connection,
										entries_title[selected].clone(),
										edited_entry,
									)
								},
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
								Ok(_) => {
									edited_entry.title = edited_entry.title.trim().to_string();
									edited_entry.description =
										edited_entry.description.trim().to_string();
									edit_entry(
										connection,
										entries_title[selected].clone(),
										edited_entry,
									);
								},
								Err(error) => println!("error: {}", error),
							}
						},
						_ => (),
					}
				},
				Err(err) => {
					println!("{}", err)
				},
			}
		},
		Some(Commands::Status) => {
			let entries = get_entries(connection).unwrap();
			let entries_title: Vec<String> = entries.clone().into_iter().map(|p| p.title).collect();
			let entries_status = &["Abandoned", "Active", "Done"];
			let selection_entries = Select::with_theme(&ColorfulTheme::default())
				.with_prompt("Pick an entry to change status")
				.items(&entries_title[..])
				.interact();
			match selection_entries {
				Ok(selected) => {
					let selection_status = Select::with_theme(&ColorfulTheme::default())
						.with_prompt("Pick updated status")
						.items(&entries_status[..])
						.interact()
						.unwrap();
					update_entry(
						connection,
						entries_title[selected].clone(),
						entries_status[selection_status].to_string(),
					);
				},
				Err(err) => {
					println!("{}", err)
				},
			}
		},

		Some(Commands::Ls(lol)) => {
			//println!("{:?}", lol);
			let entries = get_entries(connection);
			let entries_title: Vec<String> =
				entries.as_ref().unwrap().clone().into_iter().map(|p| p.title).collect();
			let selection_entries = Select::with_theme(&ColorfulTheme::default())
				.with_prompt("Pick an entry to read description")
				.items(&entries_title[..])
				.interact();
			match selection_entries {
				Ok(selected) => {
					println!(
						"description: {}",
						entries.as_ref().unwrap().clone()[selected].description
					);
					println!(
						"title: {}, status : {}",
						entries.as_ref().unwrap().clone()[selected].title,
						entries.as_ref().unwrap()[selected].status
					);
				},
				Err(err) => {
					println!("{}", err)
				},
			}
		},
		None => {},
	};
}
