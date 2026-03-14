//! Gestionnaires de routes HTTP (handlers).

use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::instrument;

/// Endpoint de health check - retourne 200 OK si le service est opérationnel.
#[instrument]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, [("Content-Type", "text/plain")], "Healthy")
}
