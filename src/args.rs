use clap::Parser;

#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct Args {
    /// Enable logging ('-v' for debug, '-vv' for tracing).
    #[arg(short = 'v', long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Print version information.
    #[arg(short = 'V', long)]
    pub version: bool,

    /// SQLite connection string.
    #[arg(long, default_value = "sqlite://cement.db", env)]
    pub db: String,
}
