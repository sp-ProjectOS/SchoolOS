extern crate diesel;
extern crate rocket;
use crate::models;
use crate::services::MessageJson;
use crate::services::Result;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};

#[derive(Serialize, Deserialize)]
pub struct NewGroup {
    pub external_id: String,
    pub name: String,
}

#[post("/group", data = "<groupq>")]
pub fn create_group(groupq: Json<NewGroup>) -> Result<Created<Json<NewGroup>>> {
    /* let connection = &mut establish_connection_pg();
    let next_id_statement = diesel::select(diesel::dsl::sql::<diesel::sql_types::BigInt>(
        "nextval('groups_auto_id_seq')",
    ));
    let next_id: i64 = next_id_statement
        .get_result(connection)
        .expect("Error loading next_id");

    let group_exists: i64 = crate::schema::groups::table
        .load(connection)
        .unwrap()
        .iter()
        .filter(|group: &&Group| group.external_id == groupq.external_id)
        .count() as i64;

    println!("group_exists: {}, next_id: {}", group_exists, next_id);

    if group_exists > 0 {
        return Err(Json(MessageJson {
            message: "Group already exists".to_string(),
        }));
    }

    let new_group = Group {
        auto_id: next_id as i64,
        external_id: groupq.external_id.clone(),
        name: groupq.name.clone(),
        created_at: std::time::SystemTime::now().into(),
        updated_at: std::time::SystemTime::now().into(),
    };

    match diesel::insert_into(crate::schema::groups::dsl::groups)
        .values(&new_group)
        .execute(connection)
    {
        Ok(_) => Ok(Created::new("/").body(groupq)),
        Err(e) => Err(Json(MessageJson {
            message: format!("Error saving new group: {}", e),
        })),
    } */

	// Example Create-Group route
	Ok(Created::new("/").body(groupq))
}

#[get("/groups")]
pub fn list_groups() -> Result<Json<Vec<models::Group>>> {
    // Example List-Groups route
    Ok(Json(vec![
        models::Group {
            auto_id: 1,
            external_id: "1".to_string(),
            name: "Group 1".to_string(),
            created_at: std::time::SystemTime::now().into(),
            updated_at: std::time::SystemTime::now().into(),
        },
        models::Group {
            auto_id: 2,
            external_id: "2".to_string(),
            name: "Group 2".to_string(),
            created_at: std::time::SystemTime::now().into(),
            updated_at: std::time::SystemTime::now().into(),
        },
        models::Group {
            auto_id: 3,
            external_id: "3".to_string(),
            name: "Group 3".to_string(),
            created_at: std::time::SystemTime::now().into(),
            updated_at: std::time::SystemTime::now().into(),
        },
    ]))
}

#[delete("/groups/<group_id>")]
pub fn delete_group(group_id: i32) -> Result<Json<MessageJson>> {
    // Example Delete-Group route
    Ok(Json(MessageJson {
        message: format!("Deleted group {}", group_id),
    }))
}
