//! Point d'entrée CLI de l'application.
//!
//! Initialise la télémétrie OpenTelemetry et démarre le serveur web si demandé.

#![warn(clippy::all)]
#![allow(dead_code)]

use clap::Parser;
use std::net::Ipv4Addr;
use std::{error::Error, future};
use tracing::{Level, info};
use tracing::{debug, error, event, span, warn};
use zero2prod::app::application::Application;
use zero2prod::otel;
use zero2prod::server::http::Webapp;

mod app;
mod server;

/// Arguments CLI parsés avec clap.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Adresse IP d'écoute
    #[arg(long, default_value = "127.0.0.0")]
    host: String,

    /// Port d'écoute
    #[arg(long, default_value_t = 8000)]
    port: u16,

    /// Active le mode serveur HTTP
    #[arg(long, default_value_t = false)]
    server: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let host: &str = args.host.as_ref();
    let ip: Ipv4Addr = host.parse()?;
    let app = Application::default().ip(ip).port(args.port);

    // Initialise RUST_LOG par défaut si non défini
    if std::env::var("RUST_LOG").is_err() {
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { std::env::set_var("RUST_LOG", "info") };
    }

    // Initialise OpenTelemetry et tracing
    let _guard = otel::init_tracing_subscriber();

    // Logs de test pour vérifier les différents niveaux
    error!("Running in CLI mode");
    warn!("Running in CLI mode");
    debug!("Running in CLI mode");
    info!("Running in CLI mode");
    info!("server: {}", args.server);

    // Création et gestion de spans manuels
    let span = span!(Level::INFO, "my span");
    span.record("key", "value");
    let _enter = span.enter();
    event!(Level::INFO, "something has happened!");
    {
        _ = span!(Level::INFO, "another span").enter();
        event!(Level::INFO, "something has happened!");
    }

    // Démarre le serveur web si --server est passé
    if args.server {
        Webapp::new(&app).serve().await?;
    }

    future::ready(Ok(())).await
}
