use rocket::config::{Config, Environment, LoggingLevel};
use rocket::http::RawStr;
use rocket::State;
use std::sync::{mpsc, Mutex};

pub type SubResult = Result<SubInfo, String>;
pub type SubTransmitter = mpsc::Sender<SubResult>;
pub type SubTxMutex<'req> = State<'req, Mutex<SubTransmitter>>;

#[get("/?<verify_token>&<challenge>&<mode>")]
fn sub_success(verify_token: &RawStr, challenge: &RawStr, mode: &RawStr, tx_mutex: SubTxMutex) {
    let tx = tx_mutex.lock().unwrap();
    tx.send(Ok(SubInfo::new(verify_token, challenge, mode)))
        .unwrap();
    println!("verify_token: {}", verify_token);
    println!("challenge: {}", challenge);
    println!("mode: {}", mode);
}

#[derive(Debug)]
pub struct SubInfo {
    pub verify_token: String,
    pub challenge: String,
    pub mode: String,
}

impl SubInfo {
    pub fn new(verify_token: &RawStr, challenge: &RawStr, mode: &RawStr) -> Self {
        Self {
            verify_token: String::from(verify_token.as_str()),
            challenge: String::from(challenge.as_str()),
            mode: String::from(mode.as_str()),
        }
    }
}

pub fn start(tx: SubTransmitter) {
    let config = Config::build(Environment::Development)
        .log_level(LoggingLevel::Off)
        .workers(1)
        .finalize()
        .unwrap();
    rocket::custom(config)
        .mount("/", routes![sub_success])
        .manage(Mutex::new(tx))
        .launch();
}
