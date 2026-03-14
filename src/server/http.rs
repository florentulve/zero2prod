//! Serveur HTTP Axum avec middleware et tracing.

use nanoid::nanoid;
use std::{error::Error, iter::once, time::Duration};

use axum::{
    Router,
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, HeaderName, Request, header::AUTHORIZATION},
    response::Response,
    routing::get,
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    trace::{self, TraceLayer},
};
use tracing::{Level, Span, info, info_span};

use crate::{app::application::Application, server::route_handler};

/// Handler racine - retourne un message statique.
async fn root() -> &'static str {
    "Hello, World!"
}

/// Construit le Router Axum avec tous les middlewares.
pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(route_handler::health_check))
        .layer(
            ServiceBuilder::new()
                .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
                .layer(CompressionLayer::new().br(true).gzip(true).zstd(true))
                .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                    "X-Request-Id",
                )))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            let id = nanoid!();
                            let matched_path = request
                                .extensions()
                                .get::<MatchedPath>()
                                .map(MatchedPath::as_str);

                            info_span!(
                                "request",
                                method = ?request.method(),
                                matched_path = matched_path,
                                uri = %request.uri(),
                                request_id = id,
                            )
                        })
                        .on_request(|_request: &Request<_>, _span: &Span| {})
                        .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                            info!("Request completed in {:?}", _latency);
                        })
                        .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {})
                        .on_eos(
                            |_trailers: Option<&HeaderMap>,
                             _stream_duration: Duration,
                             _span: &Span| {
                                // Fin du stream
                            },
                        )
                        .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR)),
                ),
        )
}

/// Wrapper pour le serveur web avec référence à la configuration.
#[derive(Debug)]
pub struct Webapp<'a> {
    app: &'a Application,
}

impl<'a> Webapp<'a> {
    /// Crée une nouvelle instance liée à l'application.
    pub fn new(app: &'a Application) -> Self {
        Self { app }
    }

    /// Démarre le serveur HTTP et attend les connexions.
    pub async fn serve(&self) -> Result<(), Box<dyn Error>> {
        let ip = self.app.bind();

        info!("🦀Zero2prod is running🦀");
        info!("Listening on http://{}", ip);

        let listener = tokio::net::TcpListener::bind(ip).await?;
        axum::serve(listener, router()).await?;
        Ok(())
    }
}
