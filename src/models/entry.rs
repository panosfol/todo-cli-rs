use crate::schema::todos;
use diesel::prelude::*;

// The struct used for todo entries
#[derive(Queryable, Default, Clone)]
pub struct Entry {
	pub id: i32,
	pub title: String,
	pub description: String,
	pub status: String,
	pub category: String,
}
#[derive(Insertable, Default)]
#[diesel(table_name=todos)]
pub struct NewEntry {
	pub title: String,
	pub description: String,
	pub status: String,
	pub category: String,
}
#[derive(Insertable, Default, AsChangeset)]
#[diesel(table_name=todos)]
pub struct EditedEntry {
	pub title: String,
	pub description: String,
	pub category: String,
}
