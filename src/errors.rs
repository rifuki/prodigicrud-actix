use actix_web::{
    http::StatusCode,
    HttpResponse,
    body::BoxBody};
use serde_json::{
    Value as JsonValue, json
};
use serde::Serialize;
use sqlx::Error as SqlxError;
use actix_web::ResponseError as ActixError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("Internal Server Error: {0}")]
    InternalServerError(JsonValue)
}

impl From<SqlxError> for AppError {
    fn from(value: SqlxError) -> Self {
        let custom_app_err_msg = CustomAppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: String::from("Database Error."),
            details: Some(value.to_string())
        };
        AppError::InternalServerError(custom_app_err_msg.into())
    }
}

impl ActixError for AppError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::InternalServerError(ref message) => HttpResponse::InternalServerError().json(message)
        }
    }
}

struct CustomAppErrorMessage<T> {
    code: u16,
    message: String,
    details: Option<T>
}

impl<T> From<CustomAppErrorMessage<T>> for JsonValue
where T: Serialize
{
    fn from(value: CustomAppErrorMessage<T>) -> Self {
        match value.details {
            Some(details) => json!({
                "success": false,
                "error": json!({
                    "code": value.code,
                    "message": value.message,
                    "details": details
                })
            }),
            None => json!({
                "success": false,
                "error": json!({
                    "code": value.code,
                    "message": value.message
                })
            })
        }
    }
}