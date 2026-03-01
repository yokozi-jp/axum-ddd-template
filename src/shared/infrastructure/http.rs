//! HTTP error handling and shared response types

use crate::shared::domain::DomainError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// API error response
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// Error code
    pub code: &'static str,
    /// Error message
    pub message: String,
    #[serde(skip)]
    status: StatusCode,
}

impl From<DomainError> for ApiError {
    fn from(e: DomainError) -> Self {
        let (code, status, message) = match &e {
            DomainError::NotFound(_) => ("NOT_FOUND", StatusCode::NOT_FOUND, e.to_string()),
            DomainError::Validation(_) => ("VALIDATION_ERROR", StatusCode::BAD_REQUEST, e.to_string()),
            DomainError::AlreadyExists(_) => ("ALREADY_EXISTS", StatusCode::CONFLICT, e.to_string()),
            DomainError::Infrastructure(_) | DomainError::Unexpected(_) => {
                // Don't leak internal details to the client
                ("INTERNAL_ERROR", StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };
        Self { code, message, status }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

/// Health check response
#[derive(Serialize)]
pub struct Health {
    /// Service status
    pub status: &'static str,
}

/// Health check handler
pub async fn health_check() -> Json<Health> {
    Json(Health { status: "ok" })
}
