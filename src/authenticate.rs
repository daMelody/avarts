use crate::{auth_server, cli};
use reqwest::Client;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::sync::mpsc;
use webbrowser;

pub type LoginResult = Result<Login, reqwest::Error>;

#[derive(Debug, Deserialize)]
pub struct Login {
    pub access_token: String,
    pub refresh_token: String,
    pub athlete: Athlete,
}

#[derive(Debug, Deserialize)]
pub struct Athlete {
    pub id: u32,
    pub username: String,
}

fn build_auth_url(id: u32) -> String {
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

async fn exchange_auth_token(code: &str, id: u32, secret: &str) -> LoginResult {
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

pub async fn login(args: &cli::Cli) {
    // build & open authentication url
    let auth_url = build_auth_url(args.id);
    if webbrowser::open(&auth_url).is_err() {
        // Allow for manual attempt
        println!("Looks like we couldn't direct you to the proper authentication portal.");
        println!("Please try the following: {}", auth_url);
    }

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        auth_server::start(tx);
    });

    match rx.recv().unwrap() {
        Ok(auth_info) => match exchange_auth_token(&auth_info.code, args.id, &args.secret).await {
            Ok(login) => {
                println!("{:#?}", login);
                println!("Scopes {:#?}", auth_info.scope);
            }
            Err(error) => {
                eprintln!("Error: {:#?}", error);
            }
        },
        Err(error) => {
            eprintln!("{}", error);
        }
    }
}
