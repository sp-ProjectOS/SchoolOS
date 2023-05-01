use crate::event_emitter::EVENT_EMITTER;
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
async fn oauth(code: String, state: String, scope: String) -> &'static str {
    // Get the code from the url
    // Send a request to the oauth2 server to get the token
    // Save the token in the storage
    // Close the window

	let config = crate::config::get_config();

    let client = reqwest::Client::new();

	let tokenurl = format!(
		"{}/oauth2/token?client_id={}&client_secret={}&code={}&redirect_uri={}&grant_type=authorization_code",
		config.oauth2.server_uri,
		config.oauth2.client_id,
		config.oauth2.client_secret,
		code,
		config.oauth2.redirect_uri
	);
    let token_res = match client
		.post(tokenurl)
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body( format!(
			"client_id={}&client_secret={}&code={}&redirect_uri={}&grant_type=authorization_code",
			config.oauth2.client_id,
			config.oauth2.client_secret,
			code,
			config.oauth2.redirect_uri
		) )
		.send()
		.await{
			Ok(r) =>{
				//println!("Token response: {}", r.status());
				r
			},
			Err(e) => {
				println!("Error getting token: {}", e);
				return "Error getting token";
			}
		};
	let (new_token, new_refresh_token) = match token_res
			.json::<serde_json::Value>()
			.await{
				Ok(r) => {
					//println!("Token response: {}", r);
					(
						r["access_token"].as_str().unwrap().to_string(),
						r["refresh_token"].as_str().unwrap().to_string()
					)
				},
				Err(e) => {
					println!("Error getting token: {}", e);
					return "Error getting token";
				}
			};
	println!("Tokens: {} {}", new_token, new_refresh_token);

	EVENT_EMITTER
        .lock()
        .unwrap()
        .emit("oauth2_code_received", ());

	"Sign in successful"
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
        .attach(AdHoc::on_liftoff("Liftoff Printer", |_| {
            Box::pin(async move {
                println!("Stalling liftoff for a second...");
                rocket::tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                println!("And we're off!");
            })
        }));
    rocket
}
