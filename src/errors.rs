// Standard lib
use std::fmt;
// External crates - Primary
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;
use deadpool_postgres::PoolError;
use tokio_postgres::error as PgError;
// External crates - Utilities
use serde::{Deserialize, Serialize};
// Other internal modules
// Const and type declarations

// **** Struct declarations ****

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    error_msg: String,
}

// Application specific Error variants. Different types of errors such as database or input param errors are converted to ApiErrors

#[derive(Debug)]
pub enum ApiError {
    PoolError(String),
    DBError(String),
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
    CannotEncodeJwtToken(String),
    CannotDecodeJwtToken(String),
}

// *** Differnt types of Type mapping & conversion Functions ***

/// Utility to convert string into an ErrorResponse with .into()
impl From<&String> for ErrorResponse {
    fn from(message: &String) -> Self {
        ErrorResponse {
            error_msg: message.into(),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            // Here you can use msg.into() or ErrorResponse::from(msg) : Both are correct
            ApiError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json::<ErrorResponse>(ErrorResponse::from(msg))
            }
            ApiError::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            ApiError::NotFound(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            ApiError::CannotEncodeJwtToken(message) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(message.into())
            }            
            ApiError::CannotDecodeJwtToken(message) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(message.into())
            }
            ApiError::DBError(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(ErrorResponse {
                    error_msg: message.into(),
                })
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
/*
impl From<PgError::DbError> for ApiError {
    fn from(error: PgError::DbError) -> ApiError {
        ApiError::DBError(error.message().into())
    }
}*/

impl From<PgError::Error> for ApiError {
    fn from(error: PgError::Error) -> ApiError {
        ApiError::DBError("Postgres Error".into())
    }
}

impl From<PoolError> for ApiError {
    fn from(error: PoolError) -> ApiError {
        ApiError::PoolError("Database connection pool error".into())
    }
}

// Utility function to print out ApiError
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}
