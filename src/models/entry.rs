use super::status::Status;
use diesel::prelude::*;

// The struct used for todo entries
#[derive(Queryable, Default)]
pub struct Entry {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: Status,
}
