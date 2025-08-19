use std::fmt;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    InvalidInput(String),
    NotFound(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::DatabaseError(err) => Some(err),
            _ => None,
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Database error occurred"
                }))
            }
            AppError::InvalidInput(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": msg
                }))
            }
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(json!({
                    "error": msg
                }))
            }
            AppError::InternalError(_) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal server error"
                }))
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}