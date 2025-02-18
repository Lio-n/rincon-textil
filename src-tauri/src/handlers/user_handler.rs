use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{auth::jwt::decode_jwt, AppState};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    id: String,
    email: String,
    name: String,
    role: String,
    avatar: String,
}

pub fn get_user_by_id(id: &str) -> Result<UserInfo, &'static str> {
    let mock_user_info = UserInfo {
  id:"jea8f498".to_string(),
  email: "leon@example.com".to_string(),
  name: "Leon".to_string(),
  role: "admin".to_string(),
  avatar: "https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=600&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTl8fGF2YXRhcnxlbnwwfHwwfHx8MA%3D%3D".to_string()
    };

    let user_found: bool = mock_user_info.id == id;

    if user_found {
        Ok(mock_user_info)
    } else {
        Err("User no found!")
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    status: u16,
    message: String,
    data: Option<UserInfo>,
}

#[tauri::command]
pub fn get_user(state: State<'_, AppState>) -> Result<UserResponse, UserResponse> {
    let token = state.token.read().unwrap(); // Acceso de solo lectura

    let decoded = match decode_jwt(&token, "asdef2f43f") {
        Ok(data) => data,
        Err(e) => {
            return Err(UserResponse {
                status: 401,
                message: format!("Error al iniciar sesión: {}", e),
                data: None,
            })
        }
    };

    let user_id = decoded.claims.sub;

    match get_user_by_id(&user_id) {
        Ok(data) => Ok(UserResponse {
            status: 200,
            message: "Login exitoso".to_string(),
            data: Some(data),
        }),
        Err(e) => Err(UserResponse {
            status: 401,
            message: format!("Error al iniciar sesión: {}", e),
            data: None,
        }),
    }
}
