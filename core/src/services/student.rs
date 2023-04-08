extern crate diesel;
extern crate rocket;
use diesel::RunQueryDsl;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use std::time;
use crate::models;
use crate::schema;
use crate::services::establish_connection_pg;
use crate::services::MessageJson;
use crate::services::Result;


#[derive(Serialize, Deserialize)]
pub struct NewStudent {
    pub external_id: String,
    pub name: String,
    pub group_id: i64,
}

#[post("/student", data = "<student>")]
pub fn create_student(student: Json<NewStudent>) -> Result<Created<Json<NewStudent>>> {
    use models::Student;
    let connection = &mut establish_connection_pg();

    let new_student = Student {
        auto_id: 1,
        external_id: student.external_id.clone(),
        name: student.name.clone(),
        group_id: student.group_id,
        created_at: time::SystemTime::now().into(),
        updated_at: time::SystemTime::now().into(),
    };

    match diesel::insert_into(self::schema::students::dsl::students)
        .values(&new_student)
        .execute(connection)
    {
        Ok(_) => Ok(Created::new("/").body(student)),
        Err(e) => Err(Json(MessageJson {
            message: format!("Error saving new student: {}", e),
        })),
    }
}

#[get("/students")]
pub fn list_students() -> Result<Json<Vec<models::Student>>> {
    use self::models::Student;
    let connection = &mut establish_connection_pg();
    let results = self::schema::students::table
        .load::<Student>(connection)
        .expect("Error loading students");
    Ok(Json(results))
}
