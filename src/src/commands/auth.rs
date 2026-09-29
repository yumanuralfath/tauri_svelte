use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthMode {
    SignIn,
    SignUp,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AuthRequest {
    mode: AuthMode,
    email: String,
    password: String,
    name: Option<String>,
}

#[tauri::command]
pub async fn authenticate(request: AuthRequest) -> Result<(), String> {
    let _ = request;
    Err("Authentication API is not configured.".into())
}
