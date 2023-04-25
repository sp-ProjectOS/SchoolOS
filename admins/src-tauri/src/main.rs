#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
mod account;
mod storage;
mod httpserver;
mod event_emitter;
use event_emitter::EVENT_EMITTER;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[rocket::main]
async fn main() {
	
	rocket::tokio::spawn(async {
		crate::httpserver::get_prebuilt().launch().await;
	});
    tauri::Builder::default() 
        .invoke_handler(tauri::generate_handler![crate::account::login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
