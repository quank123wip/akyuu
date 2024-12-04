use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

// 定义一个简单的错误枚举
#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    NotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::DatabaseError(ref msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound => write!(f, "404 Resource not found"),
        }
    }
}

// 实现IntoResponse trait以便自定义错误可以被自动转换为HTTP响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };

        let body = format!("{}", self);
        
        (status_code, body).into_response()
    }
}