pub mod jwt;
use jwt::generate_jwt;
use serde::Serialize;
use std::error::Error;
use tauri::State;

use crate::AppState;

#[derive(Serialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct MockUserDb {
    id: String,
    email: String,
    password: String, // must be encrypted
}

pub fn login(login_info: LoginInfo, state: State<'_, AppState>) -> Result<String, Box<dyn Error>> {
    let mock_db_user = MockUserDb {
        id: "jea8f498".to_string(),
        email: "leon@example.com".to_string(),
        password: "123".to_string(),
    };

    let is_authenticated: bool;

    is_authenticated =
        mock_db_user.email == login_info.email && mock_db_user.password == login_info.password;

    if is_authenticated {
        return match generate_jwt(mock_db_user.id, "asdef2f43f".to_string()) {
            Ok(access_token) => {
                let mut token = state.token.write().unwrap(); // Obtener acceso mutable
                *token = access_token.clone(); // Guardar el token
                Ok(access_token)
            }
            Err(e) => Err(format!("Error al generar el token: {}", e).into()),
        };
    }

    Err("Credenciales incorrectas".to_string().into())
}
