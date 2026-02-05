use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Kasm API error: {0}")]
    KasmError(String),
    
    #[error("Agent execution error: {0}")]
    AgentError(String),
    
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ServerError::KasmError(msg) => (StatusCode::BAD_GATEWAY, msg),
            ServerError::AgentError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ServerError::WebSocketError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ServerError::TaskNotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ServerError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ServerError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let error_response = ErrorResponse {
            error: status.canonical_reason().unwrap_or("Unknown Error").to_string(),
            message,
        };

        (status, Json(error_response)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ServerError>;
