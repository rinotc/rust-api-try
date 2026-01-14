use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub struct ApiError(pub anyhow::Error);

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        error!("Internal Server error: {:?}", self.0);

        let body = Json(json!({
            "error": "Internal Server Error",
            "message": "An unexpected error occurred on the server."
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
