#![warn(clippy::all)]
use clap::Parser;
use std::net::Ipv4Addr;
use std::{error::Error, future};
use tracing::{debug, error, span, warn};
use tracing::{info, subscriber, Level};
use tracing_subscriber::EnvFilter;
use zero2prod::app::application::Application;
use zero2prod::webapp::server::Webapp;

mod app;
mod webapp;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "127.0.0.0")]
    host: String,

    #[arg(long, default_value_t = 8000)]
    port: u16,

    /// Number of times to greet
    #[arg(long, default_value_t = false)]
    server: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let host: &str = args.host.as_ref();
    let ip: Ipv4Addr = host.parse()?;
    let app = Application::default().ip(ip).port(args.port);

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    error!("Running in CLI mode");
    warn!("Running in CLI mode");
    debug!("Running in CLI mode");
    info!("Running in CLI mode");
    info!("server: {}", args.server);

    if args.server {
        Webapp::new(&app).web().await?;
    }

    future::ready(Ok(())).await
}
