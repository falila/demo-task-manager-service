use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Debug)]
pub struct AppError {
    pub error_type: AppErrorType,
    pub message: Option<String>,
    pub cause: Option<String>,
}
#[derive(Debug)]
pub enum AppErrorType {
    NotFoundError,
    DbError,
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl AppError {
    fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                cause: _,
                error_type: _,
            } => message.clone(),
            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::NotFoundError,
            } => "Item was not found".to_string(),
            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::DbError,
            } => "Db error".to_string(),
           
        }
    }

    pub fn db_error(error: impl ToString) -> AppError { 
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?})",
            self.error_type, self.message, self.cause
        )
    }
}

impl std::fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.error_type {
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}
