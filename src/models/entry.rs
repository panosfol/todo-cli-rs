use crate::schema::todos;
use diesel::prelude::*;

/// Struct used for todo entries
#[derive(Queryable, Default, Clone)]
pub struct Entry {
	pub id: i32,
	pub title: String,
	pub description: String,
	pub status: String,
	pub category: String,
}
/// Struct used for a new entry. Id is omitted as it is provided automatically by Mysql
#[derive(Insertable, Default)]
#[diesel(table_name=todos)]
pub struct NewEntry {
	pub title: String,
	pub description: String,
	pub status: String,
	pub category: String,
}
/// Struct used for updating an entry. Status is updated through a different function and doesn't need a struct
#[derive(Insertable, Default, AsChangeset)]
#[diesel(table_name=todos)]
pub struct EditedEntry {
	pub title: String,
	pub description: String,
	pub category: String,
}
