mod auth;
mod handlers;

use handlers::auth_handler::get_token;
use handlers::user_handler::get_user;
use std::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    pub token: RwLock<String>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_token, get_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// log::info!()
