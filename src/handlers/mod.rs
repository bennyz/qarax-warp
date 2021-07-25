use std::{convert::Infallible, fmt::Display};

use serde::{Serialize, Serializer};
use serde_json::json;
use warp::http::{response, StatusCode};

use super::models;

mod ansible;
pub mod hosts;

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum QaraxResponse<T, E> {
    Success(T),
    Error(E),
}

struct ApiResponse<T, E> {
    response: QaraxResponse<T, E>,
    code: warp::http::StatusCode,
}

impl<T, E> warp::Reply for ApiResponse<T, E>
where
    T: Send + Sync + Serialize,
    E: Send + Sync + Serialize,
{
    fn into_response(self) -> warp::reply::Response {
        let response_key = match self.response {
            QaraxResponse::Success(_) => "success",
            QaraxResponse::Error(_) => "error",
        };

        response::Builder::new()
            .header("Content-Type", "application/json")
            .status(self.code)
            .body(json!({ response_key: self.response }).to_string().into())
            .unwrap()
    }
}
