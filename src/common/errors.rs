use std::fmt::{Display, Formatter, Result};

use actix_web::{HttpResponse, ResponseError};
use chrono::Utc;
use diesel::result::Error as DieselError;
use jsonwebtoken::errors::Error as JwtError;
use serde_json::json;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ErrorType {
    DatabaseError(DieselError),
    AuthenticationError,
    UserError,
}

impl Display for ErrorType {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct UserManagementError {
    pub message: String,
    pub error_type: ErrorType,
}

impl Display for UserManagementError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{:?}", self)
    }
}

impl ResponseError for UserManagementError {
    fn error_response(&self) -> HttpResponse {
        log::error!("Error occurred with message: {}", self.message);
        let response =
            json!({ "error": self.message.to_string(), "timestamp": Utc::now().to_string() });

        match self.error_type {
            ErrorType::AuthenticationError => HttpResponse::Unauthorized().json(response),
            _ => HttpResponse::InternalServerError().json(response),
        }
    }
}

impl From<DieselError> for UserManagementError {
    fn from(error: DieselError) -> UserManagementError {
        UserManagementError {
            message: error.to_string(),
            error_type: ErrorType::DatabaseError(error),
        }
    }
}

impl From<JwtError> for UserManagementError {
    fn from(error: JwtError) -> UserManagementError {
        UserManagementError {
            message: error.to_string(),
            error_type: ErrorType::AuthenticationError,
        }
    }
}
