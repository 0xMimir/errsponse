use chrono::{NaiveDateTime, Utc};
pub use errsponse_derive::ErrorResponse;
use http::StatusCode;
use serde_json::Value;

/// Reexports so that macros can use them
pub use http;
pub use serde_json;

pub use errsponse_derive::ErrorResponse as Response;

#[derive(Debug)]
pub struct ErrorResponse {
    pub status: StatusCode,
    pub message: String,
    pub cause: Value,
    pub time: NaiveDateTime,
}

pub trait ImplErrorResponse {
    fn status_code(&self) -> StatusCode;
    fn cause(&self) -> Value;
    fn time(&self) -> NaiveDateTime {
        Utc::now().naive_utc()
    }

    fn to_response(&self) -> ErrorResponse {
        let status = self.status_code();

        let message = status
            .canonical_reason()
            .map(|cause| cause.to_owned())
            .unwrap_or_default();

        let cause = self.cause();
        let time = self.time();

        ErrorResponse {
            status,
            message,
            cause,
            time,
        }
    }
}
