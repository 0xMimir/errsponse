#![allow(unused)]
use errsponse::{ImplErrorResponse, Response};
use http::StatusCode;
use serde::Serialize;
use serde_json::Value;

struct SerdeError;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct InternalError {
    additional_data: String,
}

#[derive(Response)]
#[response(default)]
enum Error {
    #[response(status = NOT_FOUND)]
    NotFound,
    #[response(status = UNAUTHORIZED)]
    Unauthorized,
    SerdeJson(SerdeError),
    #[response(cause = "{field}")]
    SomeError {
        field: String,
    },
    #[response(json)]
    InternalError(InternalError),
    #[response(cause = "{value:#?}")]
    OtherInternal(InternalError),
    #[response(nested)]
    Nested(NestedError)
}

#[derive(Response)]
enum NestedError {
    #[response(status = MOVED_PERMANENTLY, cause = "This has been moved")]
    Moved,
    #[response(status = CONFLICT,cause = "Username used")]
    Conflict,
    #[response(status = FORBIDDEN,cause = "You will not pass" )]
    Forbidden,
}

#[test]
fn response() {
    let error = Error::NotFound;
    let response = ErrorResponse::from(error);
    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.message, "Not Found");
    assert_eq!(response.cause, Value::Null);

    let error = Error::Unauthorized;
    let response = ErrorResponse::from(error);
    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
    assert_eq!(response.message, "Unauthorized");
    assert_eq!(response.cause, Value::Null);

    let error = Error::SerdeJson(SerdeError);
    let response = ErrorResponse::from(error);
    assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(response.message, "Internal Server Error");
    assert_eq!(response.cause, Value::Null);

    let error = Error::SomeError {
        field: "You are a teapot".to_owned(),
    };
    let response = ErrorResponse::from(error);
    assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(response.message, "Internal Server Error");
    assert_eq!(response.cause, Value::String("You are a teapot".to_owned()));

    let cause = InternalError {
        additional_data: "some thing went wrong".to_owned(),
    };
    let cause_as_value = serde_json::to_value(&cause).expect("How did we get here");
    let error = Error::InternalError(cause.clone());
    let response = ErrorResponse::from(error);
    assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(response.message, "Internal Server Error");
    assert_eq!(response.cause, cause_as_value);

    let error = Error::OtherInternal(cause.clone());
    let response = ErrorResponse::from(error);
    assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(response.message, "Internal Server Error");
    assert_eq!(response.cause, format!("{:#?}", cause));

    let error = Error::Nested(NestedError::Moved);
    let response = ErrorResponse::from(error);
    assert_eq!(response.status, StatusCode::MOVED_PERMANENTLY);
    assert_eq!(response.cause, Value::String("This has been moved".to_owned()));

    let error = Error::Nested(NestedError::Conflict);
    let response = ErrorResponse::from(error);
    assert_eq!(response.status, StatusCode::CONFLICT);
    assert_eq!(response.cause, Value::String("Username used".to_owned()));

    let error = Error::Nested(NestedError::Forbidden);
    let response = ErrorResponse::from(error);
    assert_eq!(response.status, StatusCode::FORBIDDEN);
    assert_eq!(response.cause, Value::String("You will not pass".to_owned()));
}