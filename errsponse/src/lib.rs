use chrono::{NaiveDateTime, Utc};
use http::StatusCode;
use serde_json::Value;

/// Reexports so that macros can use them
pub use http;
pub use serde_json;

pub use errsponse_derive::ErrorResponse as Response;

pub trait ImplErrorResponse {
    fn status_code(&self) -> StatusCode;
    fn cause(&self) -> Value;
    fn time(&self) -> NaiveDateTime {
        Utc::now().naive_utc()
    }
    fn message(&self) -> String {
        self.status_code()
            .canonical_reason()
            .map(|cause| cause.to_owned())
            .unwrap_or_default()
    }
}
