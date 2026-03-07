#![warn(clippy::all)]
use clap::Parser;
use opentelemetry::trace::{Tracer, TracerProvider as _};
use opentelemetry_sdk::trace::TracerProvider;
use std::net::Ipv4Addr;
use std::{error::Error, future};
use tracing::instrument::WithSubscriber;
use tracing::{debug, error, event, span, trace, warn};
use tracing::{info, subscriber, Level};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;
use tracing_subscriber::{fmt, EnvFilter};
use zero2prod::app::application::Application;
use zero2prod::otel;
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
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { std::env::set_var("RUST_LOG", "info") };
    }
    let _guard = otel::init_tracing_subscriber();

    error!("Running in CLI mode");
    warn!("Running in CLI mode");
    debug!("Running in CLI mode");
    info!("Running in CLI mode");
    info!("server: {}", args.server);

    let span = span!(Level::INFO, "my span");
    span.record("key", "value");
    let _enter = span.enter();
    event!(Level::INFO, "something has happened!");
    {
        _ = span!(Level::INFO, "another span").enter();
        event!(Level::INFO, "something has happened!");
    }
    if args.server {
        Webapp::new(&app).web().await?;
    }

    future::ready(Ok(())).await
}
