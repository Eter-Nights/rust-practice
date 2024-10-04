use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum TutorError {
    DBError(String),
    ActixError(String),
    InvalidInput(String),
    NotFound(String),
}

impl TutorError {
    fn error_response(&self) -> String {
        match self {
            TutorError::DBError(msg) => {
                println!("DB error: {:?}", msg);
                "Database error".into()
            }
            TutorError::ActixError(msg) => {
                println!("Actix error: {:?}", msg);
                "Actix error".into()
            }
            TutorError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
            TutorError::NotFound(msg) => {
                println!("Not found error: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl From<actix_web::Error> for TutorError {
    fn from(err: actix_web::Error) -> Self {
        TutorError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for TutorError {
    fn from(err: SQLxError) -> Self {
        TutorError::DBError(err.to_string())
    }
}

impl fmt::Display for TutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Serialize)]
struct TutorErrorResponse {
    error_msg: String,
}

impl error::ResponseError for TutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            TutorError::DBError(_) | TutorError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TutorError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            TutorError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(TutorErrorResponse {
            error_msg: self.error_response()
        })
    }
}
