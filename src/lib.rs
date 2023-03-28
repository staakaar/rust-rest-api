pub mod routes;
pub mod controllers;
pub mod application;
pub mod domain;
pub mod models;
pub mod schema;

use std::{env, sync::Arc, thread};
use mysql::{prelude::*, Opts, OptsBuilder};
use r2d2_mysql::MySqlConnectionManager;
use diesel::mysql::{MysqlConnection, self};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

   // 環境変数の取得
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let opts = Opts::from_url(&database_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MySqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

    let mut tasks = vec![];

   MysqlConnection::establish(&database_url)
       .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}