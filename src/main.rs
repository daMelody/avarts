#![feature(proc_macro_hygiene, decl_macro)]

use structopt::StructOpt;

#[macro_use]
extern crate rocket;
extern crate serde_derive;

mod auth_server;
mod authenticate;
mod cli;
mod sub_server;
mod subscribe;

#[tokio::main]
async fn main() {
    // get input from cli
    let args = cli::Cli::from_args();
    // log into Strava API
    authenticate::login(&args).await;
    match args.cmd {
        cli::Command::Grab => unimplemented!(),
        cli::Command::Watch => subscribe::connect(&args).await,
    }
}
