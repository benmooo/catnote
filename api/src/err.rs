use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use juniper::{FieldError, IntoFieldError};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppErr {
    #[error("db error! message: `{0}`")]
    DatabaseError(String),

    #[error("resouce not found: `{0}`")]
    ResourceNotFound(String),

    #[error("bson encoding error")]
    BsonEncodingError,

    #[error("`{0}`")]
    DocumentDeserializingError(String),

    #[error("hash password error")]
    HashPasswordError,

    #[error("parse hashed-password error")]
    ParseHashError,

    #[error("jwt encoding error")]
    JwtEncodingError,

    #[error("invalid token")]
    InvalidToken,

    #[error("expired token")]
    ExpiredToken,

    #[error("unauthorized")]
    Unauthorized,

    #[error("validation error: `{0}`")]
    ValidationError(String),

    #[error("parse email error")]
    ParseEmailError,

    #[error("create email failed")]
    CreateEmailFailed,

    #[error("sending email fail: `{0}`")]
    SendEmailFailed(String),

    #[error("incorrect verification code")]
    IncorrectVerificationCode,

    #[error("email password do not match")]
    EmailPasswordNotMatch,

    #[error("email not verified")]
    EmailNotVerified,

    #[error("auth middleware error")]
    AuthMiddlewareError,

    #[error("inserted id is not object id")]
    InsertIdIsNotObjectId,

    #[error("invalid object id")]
    InvalidObjectId,

    #[error("auth information extraction error")]
    AuthInfoExtractionError,
}

impl AppErr {
}

impl<S> IntoFieldError<S> for AppErr {
    fn into_field_error(self) -> juniper::FieldError<S> {
        FieldError::new(self.to_string(), juniper::Value::Null)
    }
}

impl ResponseError for AppErr {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(json!({"code": 0, "errors": vec![self.to_string()] }))
    }
}

impl From<ValidationErrors> for AppErr {
    fn from(err: ValidationErrors) -> Self {
        Self::ValidationError(err.to_string())
    }
}


pub type AppResult<T> = Result<T, AppErr>;