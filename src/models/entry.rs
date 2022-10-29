use super::status::Status;
use diesel::prelude::*;
use crate::schema::todos;

// The struct used for todo entries
#[derive(Queryable, Default)]
pub struct Entry {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
}
#[derive(Insertable, Queryable)]
#[diesel(table_name=todos)]
pub struct NewEntry {
    pub title: String,
    pub description: String,
    pub status: String,
}
