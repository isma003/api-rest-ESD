use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Recurso no encontrado")]
    NotFound,

    #[error("Solicitud incorrecta: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno del servidor".to_string(),
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Recurso no encontrado".to_string(),
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}