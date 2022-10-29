use crate::models::entry::{Entry, NewEntry};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn create_entry(conn: &mut MysqlConnection, title: String, description: String, status: String) -> Entry {
    use crate::schema::todos;

    let new_entry = NewEntry { title, description, status };

    diesel::insert_into(todos::table)
        .values(&new_entry)
        .execute(conn)
        .expect("Error saving new post");

    todos::table.order(todos::id.desc()).first(conn).unwrap()
}
