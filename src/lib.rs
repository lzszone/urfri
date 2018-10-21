#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod utils;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::{User, NewUser};

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
    .expect(&format!("Error connectiong to {}", database_url))
}

