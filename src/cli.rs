use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "enable your data-paranoid, athletic self")]
pub struct Cli {
    /// the client id belonging to the `avarts` app
    #[structopt(short, long)]
    pub id: u32,
    /// the secret id belonging to the `avarts` app
    #[structopt(short, long)]
    pub secret: String,
    /// carry out the desired functionality
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Manually get some amount of activities
    Grab,
    /// Initialize webhook subscription
    Watch,
}
