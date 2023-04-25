use serde::{Deserialize, Serialize};
use tauri::Runtime;
use tauri::Manager;
use crate::event_emitter::EVENT_EMITTER;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDetails {
    pub token: String,
    pub cached_name: Option<String>,
    pub cached_avatar_url: Option<String>,
}

impl AccountDetails {
    pub fn new(token: String) -> Self {
        Self {
            token,
            cached_name: None,
            cached_avatar_url: None,
        }
    }
}

#[tauri::command]
pub async fn login<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) {
	// Create a new window
    // Use Oauth2 to get the token
    // Save the token in the storage

	//Get oauth url from the oauth2 server using request
	
	/* let client = reqwest::Client::new(); */

	/* let oauthurl_res = match client
        .get("")
        .send()
        .await{
			Ok(r) => r,
			Err(e) => {
				println!("Error getting URL: {}", e);
				return;
			}
		};

        let oauth_url = match oauthurl_res
            .text()
            .await{
				Ok(r) => r,
				Err(e) => {
					println!("Error getting URL: {}", e);
					return;
				}
			};
	
	println!("Oauth URL: {}", oauth_url); */

    let new_window = match tauri::WindowBuilder::new(
        &app,
        "oauth2signin",
        tauri::WindowUrl::External("https://oauth2.our-space.xyz/oauth2/auth?client_id=bafd0b60-6dee-4c9f-8876-8efa4de1e7e5&response_type=code&redirect_uri=http://localhost:4445/oauth&scope=id%20profile%20email&state=12345678".parse().unwrap()),
    ).center()
    .build(){
		Ok(w) => w,
		Err(e) => {
			println!("Error creating window: {}", e);
			return;
		}
	};
}

