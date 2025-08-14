// ==========================
// FILE: src/errors.rs
// ==========================

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

// =====================
// Errors: ApiError
// =====================
#[derive(Debug)]
pub enum ApiError {
    NotFound,
    BadRequest(String),
    Db(sqlx::Error),
    Other(anyhow::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::Db(e)
    }
}
impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        ApiError::Other(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => {
                (StatusCode::NOT_FOUND, Json(json!({"error":"not found"}))).into_response()
            }
            ApiError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, Json(json!({"error": msg}))).into_response()
            }
            ApiError::Db(e) => {
                eprintln!("DB error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error":"database error"})),
                )
                    .into_response()
            }
            ApiError::Other(e) => {
                eprintln!("Internal error: {e:#}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error":"internal error"})),
                )
                    .into_response()
            }
        }
    }
}
