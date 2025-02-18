use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    auth::{login, LoginInfo},
    AppState,
};

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    status: u16,
    message: String,
    access_token: Option<String>,
}

#[tauri::command]
pub fn get_token(
    email: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<AuthResponse, AuthResponse> {
    let login_info = LoginInfo { email, password };

    match login(login_info, state) {
        Ok(access_token) => Ok(AuthResponse {
            status: 200,
            message: "Login exitoso".to_string(),
            access_token: Some(access_token),
        }),
        Err(e) => Err(AuthResponse {
            status: 401,
            message: format!("Error al iniciar sesión: {}", e),
            access_token: None,
        }),
    }
}
