use diesel::{prelude::*, sql_types::{Integer, Datetime}};

#[derive(Queryable)]
pub struct Articles {
    pub id: Integer,
    pub title: String,
    pub description: String,
    pub genre: String,
    pub state: String,
    pub created_at: Datetime,
    pub updated_at: Datetime
}