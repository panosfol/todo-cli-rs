mod db;
mod models;
mod schema;
mod util;

use clap::{Parser, Subcommand};
use db::{
	create_entry, delete_entry, edit_entry, establish_connection, get_entries,
	get_entries_with_flag, update_entry,
};
use dialoguer::{theme::ColorfulTheme, Select};
use models::entry::{EditedEntry, NewEntry};
use std::io;
use util::uppercase_converter;

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
	/// Submit a todo entry. Type --help for additional options
	New(Flag),
	/// Read all the entries. Type --help for additional options
	#[clap(alias = "list")]
	Ls(Flag),
	/// Edit a specific entry
	Edit,
	/// Delete a specific entry
	#[clap(alias = "del")]
	Delete,
	/// Update the status of an entry
	Status,
}
#[derive(Parser, Debug)]
pub struct Flag {
	///Fetch only the entries type <CATEGORY>
	#[arg(short, long)]
	category: Option<String>,
	///Fetch only the entries with status <STATUS>
	#[arg(short, long)]
	status: Option<String>,
}

fn main() {
	let connection = &mut establish_connection();
	let cli = Cli::parse();
	//This is where the main function of the app takes place
	match cli.command {
		Some(Commands::New(flag)) => {
			let mut new_entry = NewEntry::default();
			//Matching the flag struct to parse the category of the new entry
			match flag.category {
				Some(mut categ) => {
					//Converting the first letter to make matching easier and consistent
					uppercase_converter(&mut categ);
					match categ.as_str() {
						"Fun" => {
							new_entry.category = categ;
						},
						"Personal" => {
							new_entry.category = categ;
						},
						"Work" => {
							new_entry.category = categ;
						},
						&_ => {
							eprintln!("Please use one of three to describe the type of entry: fun, personal or work");
							std::process::exit(1);
						},
					}
				},
				//If the user doesn't input -c <Category> "Other" is applied automatically
				None => {
					new_entry.category = "Other".to_string();
				},
			}
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
			new_entry.title = new_entry.title.trim().to_string();
			new_entry.description = new_entry.description.trim().to_string();
			//Status is automatically "Active" for every new entry
			new_entry.status = "Active".to_string();
			create_entry(connection, new_entry);
		},

		Some(Commands::Delete) => {
			//Fetching all the entries to develop the selectable menu
			let entries = get_entries(connection);
			//Matching the "entries" vector because the load function of the diesel returns result
			match entries {
				Ok(entries) => {
					//Gathering the titles in a vector and using that vector for the selectable menu
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
				Err(error) => {
					println!("{}", error);
				},
			}
		},
		Some(Commands::Edit) => {
			let entries = get_entries(connection);
			match entries {
				Ok(entries) => {
					let mut edited_entry = EditedEntry::default();
					let entries_title: Vec<String> =
						entries.clone().into_iter().map(|p| p.title).collect();
					//Selectable fields are hardcoded, so that the entry fields will have consistency
					let selectable_fields = &["Title", "Description", "Category", "All"];
					let selectable_categories = &["Fun", "Personal", "Work"];
					let selection_entries = Select::with_theme(&ColorfulTheme::default())
						.with_prompt("Pick an entry to edit")
						.items(&entries_title[..])
						.interact();
					match selection_entries {
						Ok(selected) => {
							/*Saving both the title and the description of the selected entry
							so that they can be used later according to the user selection*/
							let selected_title = &entries[selected].title;
							let selected_desc = &entries[selected].description;
							//Developing a selectable menu of the selectable fields
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
											/*The edited_entry struct holds the new title, but the already existing description,
											so that the title will be updated but the description can stay the same*/
											edited_entry.title =
												edited_entry.title.trim().to_string();
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
											/*The edited_entry struct holds the new description, but the already existing title,
											so that the description will be updated but the title can stay the same*/
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
									let selection_category =
										Select::with_theme(&ColorfulTheme::default())
											.with_prompt("Pick an entry to change category")
											.items(&selectable_categories[..])
											.interact()
											.unwrap();
									match selection_category {
										0 => edited_entry.category = "Fun".to_string(),
										1 => edited_entry.category = "Personal".to_string(),
										2 => edited_entry.category = "Work".to_string(),
										_ => (),
									}

									edited_entry.title = selected_title.to_string();
									edited_entry.description = selected_desc.to_string();
									edit_entry(
										connection,
										entries_title[selected].clone(),
										edited_entry,
									)
								},

								3 => {
									println!("Give new title for chosen entry:");
									match io::stdin().read_line(&mut edited_entry.title) {
										Ok(_) => {},
										Err(error) => println!("error: {}", error),
									}
									println!("Give new description for chosen entry:");
									match io::stdin().read_line(&mut edited_entry.description) {
										Ok(_) => {
											edited_entry.title =
												edited_entry.title.trim().to_string();
											edited_entry.description =
												edited_entry.description.trim().to_string();
										},
										Err(error) => println!("error: {}", error),
									}
									println!("Pick new category: ");
									let selection_category =
										Select::with_theme(&ColorfulTheme::default())
											.with_prompt("Pick an entry to change category")
											.items(&selectable_categories[..])
											.interact()
											.unwrap();
									match selection_category {
										0 => edited_entry.category = "Fun".to_string(),
										1 => edited_entry.category = "Personal".to_string(),
										2 => edited_entry.category = "Work".to_string(),
										_ => (),
									}
									edit_entry(
										connection,
										entries_title[selected].clone(),
										edited_entry,
									);
								},
								_ => (),
							}
						},
						Err(err) => {
							println!("{}", err)
						},
					}
				},
				Err(error) => {
					println!("{}", error);
				},
			}
		},

		Some(Commands::Status) => {
			let entries = get_entries(connection);
			match entries {
				Ok(entries) => {
					let entries_title: Vec<String> =
						entries.clone().into_iter().map(|p| p.title).collect();
					//Hardcoding the status types for consistency and uniformity
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
				Err(error) => {
					println!("{}", error);
				},
			}
		},

		Some(Commands::Ls(flag)) => {
			//Having <flag> as argument filters through the given category and status with -c and -s respectively
			let entries = get_entries_with_flag(connection, flag);
			match entries {
				Ok(entries) => {
					let entries_title: Vec<String> =
						entries.clone().into_iter().map(|p| p.title).collect();
					let selection_entries = Select::with_theme(&ColorfulTheme::default())
						.with_prompt("Pick an entry to read description")
						.items(&entries_title[..])
						.interact();
					match selection_entries {
						Ok(selected) => {
							println!("Description: {}", entries.clone()[selected].description);
							println!(
								"Title: {}, Status: {}, Category: {}",
								entries[selected].title,
								entries[selected].status,
								entries[selected].category,
							);
						},
						Err(err) => {
							println!("{}", err)
						},
					}
				},
				Err(error) => {
					println!("{}", error);
				},
			}
		},
		None => {},
	};
}
