use crate::{cli, sub_server};
use reqwest::Client;
use serde_derive::Deserialize;
use std::sync::mpsc;

pub type SubscribeResult = Result<Subscribe, reqwest::Error>;

#[derive(Debug, Deserialize)]
pub struct Subscribe {}

pub fn build_subscribe_url(id: u32, secret: &str) -> String {
    let params = [
        format!("client_id={}", id),
        format!("client_secret={}", secret),
        String::from("callback_url=http://localhost:8000"),
        String::from("verify_token=AVARTS"),
    ]
    .join("&");
    format!(
        "https://www.strava.com/api/v3/push_subscriptions?{}",
        params
    )
}

pub async fn exchange_subscribe_token(url: String) -> SubscribeResult {
    let res = Client::new().post(url).send().await?.error_for_status()?;
    Ok(res.json().await?)
}

pub async fn connect(args: &cli::Cli) {
    // build & send request to Strava API subscriber endpoint
    let subscribe_url = build_subscribe_url(args.id, &args.secret);
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        sub_server::start(tx);
    });

    match rx.recv().unwrap() {
        Ok(sub_info) => match exchange_subscribe_token(subscribe_url).await {
            Ok(subscribe) => {
                println!("{:#?}", subscribe);
                println!("{:#?}", sub_info)
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
