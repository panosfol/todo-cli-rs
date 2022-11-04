use crate::{
	models::entry::{EditedEntry, Entry, NewEntry},
	schema::todos::dsl,
};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	MysqlConnection::establish(&database_url)
		.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn create_entry(conn: &mut MysqlConnection, new_entry: NewEntry) -> () {
	diesel::insert_into(dsl::todos)
		.values(&new_entry)
		.execute(conn)
		.expect("Error saving new post");
}

pub fn delete_entry(conn: &mut MysqlConnection, entry_title: String) -> () {
	diesel::delete(dsl::todos.filter(dsl::title.eq(entry_title)))
		.execute(conn)
		.unwrap();
}

pub fn get_entries(conn: &mut MysqlConnection) -> Result<Vec<Entry>, diesel::result::Error> {
	dsl::todos.load::<Entry>(conn)
}

pub fn update_entry(conn: &mut MysqlConnection, entry_title: String, status: String) -> () {
	diesel::update(dsl::todos)
		.filter(dsl::title.eq(entry_title))
		.set(dsl::status.eq(status))
		.execute(conn)
		.unwrap();
}
pub fn edit_entry(conn: &mut MysqlConnection, entry_title: String, new_entry: EditedEntry) -> () {
	diesel::update(dsl::todos)
		.filter(dsl::title.eq(entry_title))
		.set::<EditedEntry>(new_entry)
		.execute(conn)
		.unwrap();
}
