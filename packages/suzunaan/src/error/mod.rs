use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    DatabaseError,
    NetworkError,
    IOError,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::DatabaseError => write!(f, "An error occurred with the database"),
            ServerError::NetworkError => write!(f, "A network error occurred"),
            ServerError::IOError => write!(f, "An I/O error occurred"),
        }
    }
}

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    NotFound,
    Unauthorized,
    AlreadyExists,
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::DatabaseError(ref msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound => write!(f, "404 Resource not found"),
            AppError::Unauthorized => write!(f, "401 Unauthorized"),
            AppError::AlreadyExists => write!(f, "Resource already exists"),
            AppError::BadRequest(ref msg) => write!(f, "Bad request: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::AlreadyExists => StatusCode::CONFLICT, 
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST, 
        };

        let body = format!("{}", self);
        
        (status_code, body).into_response()
    }
}