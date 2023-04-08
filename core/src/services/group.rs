extern crate diesel;
extern crate rocket;
use crate::models;
use crate::models::Group;
use crate::services::establish_connection_pg;
use crate::services::MessageJson;
use crate::services::Result;
use diesel::RunQueryDsl;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use std::iter::Iterator;

#[derive(Serialize, Deserialize)]
pub struct NewGroup {
    pub external_id: String,
    pub name: String,
}

#[post("/group", data = "<groupq>")]
pub fn create_group(groupq: Json<NewGroup>) -> Result<Created<Json<NewGroup>>> {
    let connection = &mut establish_connection_pg();
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
    }
}

#[get("/groups")]
pub fn list_groups() -> Result<Json<Vec<models::Group>>> {
    use self::models::Group;
    let connection = &mut establish_connection_pg();
    let results = crate::schema::groups::table
        .load::<Group>(connection)
        .expect("Error loading groups");
    Ok(Json(results))
}

#[delete("/groups/<group_id>")]
pub fn delete_group(group_id: i32) -> Result<Json<MessageJson>> {
    let connection = &mut establish_connection_pg();
    match diesel::delete(
		crate::schema::groups::dsl::groups.load(connection).unwrap()
		.iter()
		.filter(|group: &&Group| group.auto_id == group_id as i64)
		.collect::<Vec<&Group>>()[0]
	)
	{
		Ok(_) => Ok(Json(MessageJson {
			message: format!("Group {} deleted", group_id),
		})),
		Err(e) => Err(Json(MessageJson {
			message: format!("Error deleting group: {}", e),
		})),
	}
}
