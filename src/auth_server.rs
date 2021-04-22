use rocket::config::{Config, Environment, LoggingLevel};
use rocket::http::RawStr;
use rocket::State;
use std::sync::{mpsc, Mutex};

pub type AuthResult = Result<AuthInfo, String>;
pub type AuthTransmitter = mpsc::Sender<AuthResult>;
pub type AuthTxMutex<'req> = State<'req, Mutex<AuthTransmitter>>;

#[get("/?<code>&<scope>")]
fn auth_success(code: &RawStr, scope: &RawStr, tx_mutex: AuthTxMutex) -> &'static str {
    let tx = tx_mutex.lock().unwrap();
    tx.send(Ok(AuthInfo::new(code, scope))).unwrap();
    "You may close this browser tab and return to the terminal."
}

#[get("/?<error>", rank = 2)]
fn error(error: &RawStr, tx_mutex: AuthTxMutex) -> String {
    let tx = tx_mutex.lock().unwrap();
    tx.send(Err(String::from(error.as_str()))).unwrap();
    format!("Error: {}, please return to the terminal.", error)
}

#[derive(Debug)]
pub struct AuthInfo {
    pub code: String,
    pub scope: Vec<String>,
}

impl AuthInfo {
    pub fn new(code: &RawStr, scopes: &RawStr) -> Self {
        Self {
            code: String::from(code.as_str()),
            scope: scopes.as_str().split(",").map(String::from).collect(),
        }
    }
}

pub fn start(tx: AuthTransmitter) {
    let config = Config::build(Environment::Development)
        .log_level(LoggingLevel::Off)
        .workers(1)
        .finalize()
        .unwrap();
    rocket::custom(config)
        .mount("/", routes![auth_success, error])
        .manage(Mutex::new(tx))
        .launch();
}
