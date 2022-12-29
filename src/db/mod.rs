use crate::{
	models::entry::{EditedEntry, Entry, NewEntry},
	schema::todos::dsl,
	Flag,
};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, fs};

///Connecting with the database that is running with Docker. Please set your .env according to the .env.example
#[cfg(feature = "local_database")]
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
	.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

///Connecting with the database that is running with Docker. If the app is being used for the first
///time, use the command <connect> with the correct url to configure the config.txt
#[cfg(not(feature = "local_database"))]
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = fs::read_to_string("./config.txt");
    MysqlConnection::establish(&database_url.as_ref().unwrap())
	.unwrap_or_else(|_| panic!("Error connecting to {}", database_url.unwrap()))
}

///Adds a new entry to the database
pub fn create_entry(conn: &mut MysqlConnection, new_entry: NewEntry) -> () {
	diesel::insert_into(dsl::todos)
		.values(&new_entry)
		.execute(conn)
		.expect("Error saving new post");
}

///Deletes an entry from the database
pub fn delete_entry(conn: &mut MysqlConnection, entry_title: String) -> () {
	diesel::delete(dsl::todos.filter(dsl::title.eq(entry_title)))
		.execute(conn)
		.unwrap();
}

///Fetches all the entries
pub fn get_entries(conn: &mut MysqlConnection) -> Result<Vec<Entry>, diesel::result::Error> {
	dsl::todos.load::<Entry>(conn)
}

///Fetches all entries, filtering with the given status and/or category of the entry
///Status and category are optional. If none provided, fetches all the entries
pub fn get_entries_with_flag(
	conn: &mut MysqlConnection,
	flag: Flag,
) -> Result<Vec<Entry>, diesel::result::Error> {
	let status = flag.status;
	let category = flag.category;
	let entries: Result<Vec<Entry>, diesel::result::Error>;

	match (status, category) {
		(Some(status), None) => {
			entries = dsl::todos.filter(dsl::status.eq(status)).load::<Entry>(conn);
		},
		(Some(status), Some(category)) => {
			entries = dsl::todos
				.filter(dsl::status.eq(status))
				.filter(dsl::category.eq(category))
				.load::<Entry>(conn);
		},
		(None, Some(category)) => {
			entries = dsl::todos.filter(dsl::category.eq(category)).load::<Entry>(conn);
		},
		(None, None) => {
			entries = dsl::todos.load::<Entry>(conn);
		},
	}
	entries
}

///Updates the status of a specific entry
pub fn update_entry(conn: &mut MysqlConnection, entry_title: String, status: String) -> () {
	diesel::update(dsl::todos)
		.filter(dsl::title.eq(entry_title))
		.set(dsl::status.eq(status))
		.execute(conn)
		.unwrap();
}

///Edits the title or the description of a specific entry
pub fn edit_entry(
	conn: &mut MysqlConnection,
	entry_title: String,
	updated_entry: EditedEntry,
) -> () {
	diesel::update(dsl::todos)
		.filter(dsl::title.eq(entry_title))
		.set::<EditedEntry>(updated_entry)
		.execute(conn)
		.unwrap();
}
