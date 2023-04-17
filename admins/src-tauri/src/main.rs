mod preferences;



#[tauri::command]
fn get_local_preferences() -> String {
    let preferences = crate::preferences::get_local_preferences();
    serde_json::to_string(&preferences).unwrap()
}

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
