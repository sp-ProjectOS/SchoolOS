use rocket::fairing::AdHoc;

#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/oauth?<code>&<state>&<scope>")]
async fn oauth(code: String, state: String, scope: String) -> Result< rocket::fs::NamedFile, rocket::response::status::NotFound<String>> {
	// Get the code from the url
	// Send a request to the oauth2 server to get the token
	// Save the token in the storage
	// Close the window
	println!("Code: {}", code);

	rocket::fs::NamedFile::open("../closewindow.html").await.map_err(|_| rocket::response::status::NotFound("Not found".to_string()))
}

pub fn get_prebuilt() -> rocket::Rocket<rocket::Build> {
	 let figment = rocket::Config::figment()
        .merge(("port", 4445))
		.merge(("address", "0.0.0.0"));
	let rocket = rocket::custom(figment)
	.mount("/", routes![index])
	.mount("/", routes![oauth])
	// Set 404 to redirect to tauri://index.html
	.register("/", catchers![not_found])
    .attach(AdHoc::on_liftoff("Liftoff Printer", |_| Box::pin(async move {
        println!("Stalling liftoff for a second...");
        rocket::tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("And we're off!");
    })));
	rocket
}