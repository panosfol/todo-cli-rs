// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Unsigned<Bigint>,
        title -> Varchar,
        description -> Text,
        status -> Tinytext,
    }
}
