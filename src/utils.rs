use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use serde::Serialize;

pub struct PrettyJson<T>(pub T);

impl<T: Serialize> IntoResponse for PrettyJson<T> {
    fn into_response(self) -> Response {
        match serde_json::to_string_pretty(&self.0) {
            Ok(pretty) => (
                StatusCode::OK,
                [(axum::http::header::CONTENT_TYPE, "application/json")],
                pretty,
            )
                .into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}