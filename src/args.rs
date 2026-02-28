use clap::Parser;

#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct Args {
    /// Enable logging ('-v' for debug, '-vv' for tracing).
    #[arg(short = 'v', long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// libSQL remote URL.
    #[arg(long, env)]
    pub db_remote: String,

    /// libSQL remote token.
    #[arg(long, env)]
    pub db_token: String,
}
