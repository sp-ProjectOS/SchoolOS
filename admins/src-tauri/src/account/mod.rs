use crate::event_emitter::EVENT_EMITTER;
use serde::{Deserialize, Serialize};
use tauri::Runtime;

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
pub async fn login<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> String {
    // Create a new window
    // Use Oauth2 to get the token
    // Save the token in the storage
	let oauth_config = crate::config::get_config().oauth2;
	let scope_str = oauth_config.scope.join("%20");
	let oauth_url = format!(
		"{}/oauth2/auth?client_id={}&response_type=code&redirect_uri={}&scope={}&state=12345678",
		oauth_config.server_uri,
		oauth_config.client_id,
		oauth_config.redirect_uri,
		scope_str
	);
    let new_window = match tauri::WindowBuilder::new(
        &app,
        "oauth2signin",
        tauri::WindowUrl::External(oauth_url.parse().unwrap()),
    )
	.center()
	.title("Sign in")
    .build(){
		Ok(w) => w,
		Err(e) => {
			println!("Error creating window: {}", e);
			return String::from("error");
		}
	};

    EVENT_EMITTER
        .lock()
        .unwrap()
        .on("oauth2_code_received", move |_: ()| {
            new_window.close().unwrap();
			window.emit("navigate_to", "dashboard").unwrap();
        });
	
	String::from("ok")
}

#[tauri::command]
pub async fn logout<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) {
	// Remove the token from the storage
	// Navigate to the login page
	window.emit("navigate_to", "/").unwrap();
}