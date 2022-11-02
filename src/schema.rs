// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Varchar,
        description -> Text,
        status -> Text,
    }
}
