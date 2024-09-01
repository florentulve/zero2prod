use std::error::Error;

use axum::{routing::get, serve::Serve, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{app::application::Application, webapp::route_handler};

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(route_handler::health_check))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

pub struct Webapp<'a> {
    app: &'a Application,
}

impl<'a> Webapp<'a> {
    pub fn new(app: &'a Application) -> Self {
        Self { app }
    }

    pub async fn web(&self) -> Result<(), Box<dyn Error>> {
        let ip = self.app.bind();

        info!("🦀Zero2prod is running🦀");
        info!("Listening on http://{}", ip);

        let listener = tokio::net::TcpListener::bind(ip).await?;
        axum::serve(listener, app()).await?;
        Ok(())
    }
}
