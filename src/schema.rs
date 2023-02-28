// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Integer,
        title -> Varchar,
        description -> Nullable<Varchar>,
        genre -> Nullable<Varchar>,
        state -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
