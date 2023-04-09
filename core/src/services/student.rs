extern crate diesel;
extern crate rocket;
use crate::models;
use crate::services::Result;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use std::time;

#[derive(Serialize, Deserialize)]
pub struct NewStudent {
    pub external_id: String,
    pub name: String,
    pub group_id: i64,
}

#[post("/student", data = "<student>")]
pub fn create_student(student: Json<NewStudent>) -> Result<Created<Json<NewStudent>>> {
    // Example Create-Student route
    Ok(Created::new("/").body(student))
}

#[get("/students")]
pub fn list_students() -> Result<Json<Vec<models::Student>>> {
    // Example List-Students route
    Ok(Json(vec![
        models::Student {
            id: 1,
            external_id: "1".to_string(),
            name: "Student 1".to_string(),
            group_id: 1,
            created_at: time::SystemTime::now().into(),
            updated_at: time::SystemTime::now().into(),
        },
        models::Student {
            id: 2,
            external_id: "2".to_string(),
            name: "Student 2".to_string(),
            group_id: 1,
            created_at: time::SystemTime::now().into(),
            updated_at: time::SystemTime::now().into(),
        },
        models::Student {
            id: 3,
            external_id: "3".to_string(),
            name: "Student 3".to_string(),
            group_id: 2,
            created_at: time::SystemTime::now().into(),
            updated_at: time::SystemTime::now().into(),
        },
    ]))
}
