use crate::util::API_URL;
use serde::Deserialize;
use tauri_plugin_http::reqwest;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AuthMode {
    SignIn,
    SignUp,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AuthRequest {
    mode: Option<AuthMode>,
    email: Option<String>,
    password: Option<String>,
    name: Option<String>,
}

#[tauri::command]
pub async fn authenticate(_request: AuthRequest) -> Result<(), String> {
    let res = reqwest::get(API_URL).await.map_err(|e| e.to_string())?;

    match res.status() {
        reqwest::StatusCode::OK => {
            println!("Authentication successful");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Authentication failed: Unauthorized");
        }
        _ => {
            println!("Authentication failed: {:?}", res.status());
        }
    }

    println!("{:?}", res);
    let body = res.text().await.map_err(|e| e.to_string())?;
    println!("Response body: {}", body);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::AuthRequest;

    #[tokio::test]
    async fn test_authenticate() {
        let api = AuthRequest::default();

        let result = super::authenticate(api).await;
        println!("result: {:?}", result);
    }
}
