use std::{error::Error, iter::once, time::Duration};

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{header::AUTHORIZATION, HeaderMap, HeaderName, Request},
    response::Response,
    routing::get,
    serve::Serve,
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    classify::ServerErrorsFailureClass,
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    trace::{self, TraceLayer},
};
use tracing::{info, info_span, Level, Span};

use crate::{app::application::Application, webapp::route_handler};

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(route_handler::health_check))
        .layer(
            ServiceBuilder::new()
                .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
                .layer(CompressionLayer::new())
                .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                    "x-request-id",
                )))
                /*.layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            // Log the matched route's path (with placeholders not filled in).
                            // Use request.uri() or OriginalUri if you want the real path.
                            let matched_path = request
                                .extensions()
                                .get::<MatchedPath>()
                                .map(MatchedPath::as_str);
                            info_span!(
                                "request",
                                method = ?request.method(),
                                uri = %request.uri(),
                            )
                        })
                        .on_request(|_request: &Request<_>, _span: &Span| {
                            // You can use `_span.record("some_other_field", value)` in one of these
                            // closures to attach a value to the initially empty field in the info_span
                            // created above.
                        })
                        .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                            // ...
                        })
                        .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                            // ...
                        })
                        .on_eos(
                            |_trailers: Option<&HeaderMap>,
                             _stream_duration: Duration,
                             _span: &Span| {
                                // ...
                            },
                        )
                        .on_failure(
                            |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                                // ...
                            },
                        ),
                ),*/
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                        .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
                ),
        )
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
