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

diesel::table! {
    reviews (id) {
        id -> Integer,
        article_id -> Integer,
        detail -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}