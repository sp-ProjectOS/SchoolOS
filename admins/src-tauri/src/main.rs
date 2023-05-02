#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
mod account;
mod event_emitter;
mod httpserver;
mod config;


#[rocket::main]
async fn main() {
    rocket::tokio::spawn(async {
		match crate::httpserver::get_prebuilt().launch().await{
			Ok(r) => r,
			Err(e) => {
				println!("Error starting http server: {}", e);
				return;
			}
		};
    });
    tauri::Builder::default()
		.plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
			crate::account::login,
			crate::account::logout
		])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
