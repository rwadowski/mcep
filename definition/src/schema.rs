// @generated automatically by Diesel CLI.

diesel::table! {
    app_definitions (id) {
        id -> Int4,
        title -> Varchar,
        version -> Varchar,
        body -> Nullable<Text>,
        description -> Nullable<Text>,
        help -> Nullable<Text>,
    }
}
