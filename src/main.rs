use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "enable your data-paranoid, athletic self")]
struct Cli {
    /// the client id belonging to the `avarts` app
    #[structopt(short, long)]
    client: String,
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

fn main() {
    let opt = Cli::from_args();
    println!("{:?}", opt);
}
