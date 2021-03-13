use actix_web::{HttpResponse, error};
use error::ResponseError;
use super::Resp;
use bcrypt::BcryptError;

/// error format "code#message"
#[derive(thiserror::Error, Debug)]
pub enum BusinessError {
    #[error("10000#An internal error occurred. Please try again later.")]
    InternalError { #[from] source: anyhow::Error },
    #[error("10001#Validation error on field: {field}")]
    ValidationError { field: String },
    #[error("10002#Argument error")]
    ArgumentError,
    #[error("10003#Authentication failure")]
    AuthenticationFailure,
    #[error("10004#Login error, invalid username or password")]
    LoginError,
    #[error("20002#database access error")]
    DatabaseAccessError { #[from] source: rbatis::Error },
    #[error("20003#bcrypt error")]
    BcryptError { #[from] source: BcryptError },
    #[error("20004#actix error")]
    ActixError{#[from] source: actix_web::Error},
}

impl BusinessError {
    fn to_code(&self) -> i32 {
        let code = &self.to_string()[0..5];
        code.parse().unwrap_or(-1)
    }

    fn to_message(&self) -> String {
        self.to_string()[6..].to_owned()
    }
}

impl ResponseError for BusinessError {
    fn error_response(&self) -> HttpResponse {
        if let BusinessError::InternalError { source } = self {
            log::error!("[InternalError] {}", source);
        }
        if self.to_code() > 20000 {
            log::error!("[SystemError] {:?}", self);
        }
        let resp = Resp::err(self.to_code(), &self.to_message());
        HttpResponse::BadRequest().json(resp)
    }
}

