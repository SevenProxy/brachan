use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid data: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
pub struct ErrResponse {
    code: u16,
    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, details) = match self {
            AppError::NotFound(resource) => (
                StatusCode::NOT_FOUND,
                "Resource not found".to_string(),
                Some(resource.clone()),
            ),

            AppError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                "Invalid Data".to_string(),
                Some(msg.clone()),
            ),

            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                "Conflict".to_string(),
                Some(msg.clone()),
            ),

            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string(), None),

            AppError::Internal(error) => {
                tracing::error!("Internal errror: {:?}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    None,
                )
            }
        };

        let body = ErrResponse {
            code: status.as_u16(),
            message,
            details,
        };

        (status, Json(body)).into_response()
    }
}
