use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

use crate::domain::services::errors::{AuthServiceError, AccountServiceError};

#[derive(Serialize)]
struct ApiError {
    message: String,
    status: u16,
}

impl ApiError {
    fn new(message: String, status: u16) -> Self {
        Self { message, status }
    }
}

// Transforming ResponseError from other layers
impl ResponseError for AccountServiceError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AccountServiceError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AccountServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            AccountServiceError::Default => StatusCode::BAD_GATEWAY,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let response = ApiError::new(self.to_string(), self.status_code().into());
        HttpResponse::build(self.status_code()).json(response)
    }
}

impl ResponseError for AuthServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthServiceError::InvalidCredentials => StatusCode::NOT_FOUND,
            AuthServiceError::AlreadyRegistered(_) => StatusCode::BAD_REQUEST,
            AuthServiceError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AuthServiceError::Validation(_) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let response = ApiError::new(self.to_string(), self.status_code().into());
        HttpResponse::build(self.status_code()).json(response)
    }
}
