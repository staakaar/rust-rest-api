pub mod routes;
pub mod controllers;
pub mod domain;
pub mod models;
pub mod schema;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
   
   // 環境変数の取得
   let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

   MysqlConnection::establish(&database_url)
       .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}