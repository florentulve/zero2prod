use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::instrument;

#[instrument(name = "health_check")]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, [("Content-Type", "text/plain")], "Healthy")
}
