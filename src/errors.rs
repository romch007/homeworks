use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::pooled_connection::bb8;

pub trait AppError: Send + fmt::Display + fmt::Debug + 'static {
    fn response(&self) -> axum::response::Response;
}

pub type BoxedAppError = Box<dyn AppError>;

impl AppError for BoxedAppError {
    fn response(&self) -> axum::response::Response {
        (**self).response()
    }
}

impl IntoResponse for BoxedAppError {
    fn into_response(self) -> axum::response::Response {
        self.response()
    }
}

pub fn not_found() -> BoxedAppError {
    custom(StatusCode::NOT_FOUND)
}

pub fn server_error() -> BoxedAppError {
    custom(StatusCode::INTERNAL_SERVER_ERROR)
}

pub type AppResult<T> = Result<T, BoxedAppError>;

impl<E: std::error::Error + Send + 'static> AppError for E {
    fn response(&self) -> axum::response::Response {
        tracing::error!("Internal Server Error: {}", self);

        server_error_response()
    }
}

impl From<bb8::RunError> for BoxedAppError {
    fn from(err: bb8::RunError) -> Self {
        Box::new(err)
    }
}

impl From<diesel::ConnectionError> for BoxedAppError {
    fn from(err: diesel::ConnectionError) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<DieselError> for BoxedAppError {
    fn from(err: DieselError) -> BoxedAppError {
        match err {
            DieselError::NotFound => not_found(),
            DieselError::DatabaseError(DatabaseErrorKind::ClosedConnection, _) => server_error(),
            _ => Box::new(err),
        }
    }
}

impl From<aws_sdk_s3::presigning::PresigningConfigError> for BoxedAppError {
    fn from(err: aws_sdk_s3::presigning::PresigningConfigError) -> Self {
        Box::new(err)
    }
}

impl<E: std::error::Error + Send + 'static> From<aws_sdk_s3::error::SdkError<E>> for BoxedAppError {
    fn from(err: aws_sdk_s3::error::SdkError<E>) -> Self {
        Box::new(err)
    }
}

#[derive(Debug, Clone)]
pub struct CustomApiError {
    status: StatusCode,
}

impl fmt::Display for CustomApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.status.fmt(f)
    }
}

impl AppError for CustomApiError {
    fn response(&self) -> Response {
        self.status.into_response()
    }
}

pub fn custom(status: StatusCode) -> BoxedAppError {
    Box::new(CustomApiError { status })
}

fn server_error_response() -> axum::response::Response {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
}
