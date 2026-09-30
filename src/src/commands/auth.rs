use crate::util::API_URL;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri_plugin_http::reqwest::Client;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AuthMode {
    SignIn,
    SignUp,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    mode: AuthMode,
    email: String,
    password: String,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub data: AuthData,
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthData {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub token_type: String,
    pub user: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub created_at: String,
    pub email: String,
    pub id: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub role: String,
    pub username: String,
}

#[tauri::command]
pub async fn authenticate(request: AuthRequest) -> Result<ApiResponse, String> {
    match request.mode {
        AuthMode::SignIn => login(request).await,
        AuthMode::SignUp => register(request).await,
    }
}

async fn register(request: AuthRequest) -> Result<ApiResponse, String> {
    let client = Client::new();

    let payload = json!({
        "email": request.email,
        "password": request.password,
        "username": request.name.ok_or("Name is required")?,
    });

    let res = client
        .post(format!("{}/api/auth/register", API_URL))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();

        return Err(format!("Register failed: {} - {}", status, body));
    }

    res.json::<ApiResponse>().await.map_err(|e| e.to_string())
}

async fn login(request: AuthRequest) -> Result<ApiResponse, String> {
    let client = Client::new();

    let payload = json!({
        "email": request.email,
        "password": request.password,
    });

    let res = client
        .post(format!("{}/api/auth/login", API_URL))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();

        return Err(format!("Login failed: {} - {}", status, body));
    }

    res.json::<ApiResponse>().await.map_err(|e| e.to_string())
}

// #[cfg(test)]
// mod tests {
//     use super::AuthRequest;
//
//     #[tokio::test]
//     async fn test_authenticate() {
//         let api = AuthRequest::default();
//
//         let result = super::authenticate(api).await;
//         println!("result: {:?}", result);
//     }
// }
