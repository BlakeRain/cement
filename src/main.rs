use app::create_app;
use args::Args;
use clap::Parser;
use env::Env;
use poem::{listener::TcpListener, Server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod args;
mod env;
mod model;
mod templates;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    {
        let fmt = tracing_subscriber::fmt::layer()
            .with_target(false)
            .without_time();
        let sub = tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(match args.verbose {
                0 => std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
                1 => "debug".into(),
                _ => "trace".into(),
            }))
            .with(fmt);
        sub.init();
    }

    let env = Env::new(&args.db).await?;
    let app = create_app(env);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}
