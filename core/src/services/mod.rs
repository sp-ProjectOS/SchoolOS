extern crate diesel;
extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};


pub mod student;
pub mod group;

#[derive(Serialize, Deserialize)]
pub struct MessageJson {
    pub message: String,
}

type Result<T, E = Json<MessageJson>> = std::result::Result<T, E>;
