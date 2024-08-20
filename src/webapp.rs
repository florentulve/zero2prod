use std::net::SocketAddr;

use axum::{routing::get, Router};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn boostrap() {
    let ip: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    // Start logging to console
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::Layer::default().compact())
        .init();
    
    info!("🦀Zero2prod is running🦀");
    info!("Listening on http://{}", ip);

    let app = Router::new()
        .route("/", get(root));
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


