extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::env;

pub mod student;
pub mod group;

pub fn establish_connection_pg() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize)]
pub struct MessageJson {
    pub message: String,
}

type Result<T, E = Json<MessageJson>> = std::result::Result<T, E>;
