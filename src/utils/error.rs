/// 自定义错误
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "token error")]
    TokenError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::BadClientData => StatusCode::BAD_REQUEST,
            CustomError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            CustomError::TokenError => StatusCode::UNAUTHORIZED,
        }
    }
}