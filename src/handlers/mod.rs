use anyhow::anyhow;
use serde::Serialize;
use serde_json::json;
use warp::http::{response, StatusCode};

use super::models;

mod ansible;
pub mod hosts;

#[derive(Debug)]
struct SuccessResponse<T> {
    data: T,
    code: warp::http::StatusCode,
}

#[derive(Debug)]
struct ErrorResponse<E> {
    error: E,
    code: warp::http::StatusCode,
}

#[derive(Debug)]
enum ApiResponse<T, E> {
    Success(SuccessResponse<T>),
    Error(ErrorResponse<E>),
}

impl<T, E> warp::Reply for ApiResponse<T, E>
where
    T: Send + Sync + Serialize,
    E: Send + Sync + Serialize,
{
    fn into_response(self) -> warp::reply::Response {
        match self {
            ApiResponse::Success(success_response) => response::Builder::new()
                .header("Content-Type", "application/json")
                .status(success_response.code)
                .body(
                    json!({ "response": success_response.data })
                        .to_string()
                        .into(),
                )
                .unwrap(),
            ApiResponse::Error(error_response) => response::Builder::new()
                .header("Content-Type", "application/json")
                .status(error_response.code)
                .body(json!({ "error": error_response.error }).to_string().into())
                .unwrap(),
        }
    }
}
