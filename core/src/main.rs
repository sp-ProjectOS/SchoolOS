#[macro_use]
extern crate rocket;
pub mod models;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![services::group::create_group])
        .mount("/", routes![services::student::create_student])
        .mount("/", routes![services::group::list_groups])
        .mount("/", routes![services::student::list_students])
		.mount("/", routes![services::group::delete_group])
}
