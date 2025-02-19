mod auth;
mod database;
mod handlers;

use database::init_db;
use handlers::auth_handler::get_token;
use handlers::user_handler::get_user;
use sqlx::SqlitePool;
use std::sync::RwLock;

pub struct AppState {
    pub token: RwLock<String>,
    pub db: RwLock<SqlitePool>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let pool = SqlitePool::connect("sqlite:./db.sqlite")
        .await
        .expect("Error al conectar a la base de datos");

    if let Err(e) = init_db(pool.clone()).await {
        eprintln!("Error inicializando la base de datos: {}", e);
    }

    let app_state = AppState {
        token: RwLock::new(String::new()), // Inicia el token como cadena vacía
        db: RwLock::new(pool),             // Usa el pool de base de datos
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_token, get_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// log::info!()
