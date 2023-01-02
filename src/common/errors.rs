use chrono::Local;
use std::fmt::{Display, Formatter, Result};

use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error;
use log::error;
use serde_json::json;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ErrorType {
    DieselError(Error),
    UserError(String),
}

#[derive(Debug)]
pub struct UserManagmenetError {
    pub message: String,
    pub error_type: ErrorType,
}

impl Display for UserManagmenetError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{:?}", self)
    }
}

impl ResponseError for UserManagmenetError {
    fn error_response(&self) -> HttpResponse {
        match &self.error_type {
            ErrorType::DieselError(error) => {
                error!("Error occurred: {} with message: {}", error, self.message)
            }
            _ => error!("Error occurred with message: {}", self.message),
        }

        HttpResponse::InternalServerError()
            .json(json!({"error": self.message.to_string(), "timestamp": Local::now().to_string()}))
    }
}

impl From<Error> for UserManagmenetError {
    fn from(error: Error) -> UserManagmenetError {
        UserManagmenetError {
            message: error.to_string(),
            error_type: ErrorType::DieselError(error),
        }
    }
}
