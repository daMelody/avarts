#![feature(proc_macro_hygiene, decl_macro)]

use std::sync::mpsc;
use structopt::StructOpt;
use webbrowser;

#[macro_use]
extern crate rocket;
extern crate serde_derive;

mod authenticate;
mod server;

#[derive(Debug, StructOpt)]
#[structopt(about = "enable your data-paranoid, athletic self")]
struct Cli {
    /// the client id belonging to the `avarts` app
    #[structopt(short, long)]
    id: u32,
    /// the secret id belonging to the `avarts` app
    #[structopt(short, long)]
    secret: String,
    /// carry out the desired functionality
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Manually get some amount of activities
    Grab {},
    Watch {},
}

#[tokio::main]
async fn main() {
    // get input from cli
    let args = Cli::from_args();
    // build & open authentication url
    let auth_url = authenticate::build_auth_url(args.id);
    if webbrowser::open(&auth_url).is_err() {
        // Allow for manual attempt
        println!("Looks like we couldn't direct you to the proper authentication portal.");
        println!("Please try the following: {}", auth_url);
    }

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        server::start(tx);
    });

    match rx.recv().unwrap() {
        Ok(auth_info) => {
            match authenticate::exchange_token(&auth_info.code, args.id, &args.secret).await {
                Ok(login) => {
                    println!("{:#?}", login);
                    println!("Scopes {:#?}", auth_info.scope);
                }
                Err(error) => eprintln!("Error: {:#?}", error),
            }
        }
        Err(error) => eprintln!("{}", error),
    }
}
