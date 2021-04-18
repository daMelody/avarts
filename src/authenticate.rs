use reqwest::Client;
use serde_derive::Deserialize;
use std::collections::HashMap;

pub type LoginResult = Result<Login, reqwest::Error>;

#[derive(Debug, Deserialize)]
pub struct Login {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn build_auth_url(id: u32) -> String {
    let scopes = ["read_all", "profile:read_all", "activity:read_all"].join(",");
    let params = [
        format!("client_id={}", id),
        String::from("redirect_uri=http://localhost:8000"),
        String::from("response_type=code"),
        String::from("approval_prompt=auto"),
        format!("scope={}", scopes),
    ]
    .join("&");
    format!("https://www.strava.com/oauth/authorize?{}", params)
}

pub async fn exchange_token(code: &str, id: u32, secret: &str) -> LoginResult {
    let mut body = HashMap::new();
    body.insert("client_id", format!("{}", id));
    body.insert("client_secret", String::from(secret));
    body.insert("code", String::from(code));
    body.insert("grant_type", String::from("authorization_code"));
    let res = Client::new()
        .post("https://www.strava.com/oauth/token")
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    Ok(res.json().await?)
}
