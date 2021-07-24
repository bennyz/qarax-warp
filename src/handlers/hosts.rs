use crate::env::Environment;

use super::*;
use models::hosts as host_model;
use models::hosts::{HostError, NewHost, Status};
use uuid::Uuid;

pub async fn list(env: Environment) -> Result<impl warp::Reply, warp::Rejection> {
    let hosts = host_model::list(env.db()).await;
    match hosts {
        Ok(hosts) => Ok(ApiResponse::Success(SuccessResponse {
            code: StatusCode::OK,
            data: hosts,
        })),
        Err(e) => Ok(ApiResponse::Error(ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            error: HostError::ErrorList(e.to_string()),
        })),
    }
}

pub async fn add(host: NewHost, env: Environment) -> Result<impl warp::Reply, warp::Rejection> {
    let host_id = host_model::add(env.db(), &host).await;
    match host_id {
        Ok(host_id) => Ok(ApiResponse::Success(SuccessResponse {
            code: StatusCode::OK,
            data: host_id,
        })),
        Err(e) => Ok(ApiResponse::Error(ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            error: HostError::ErrorList(e.to_string()),
        })),
    }
}

pub async fn install(host_id: Uuid, env: Environment) -> Result<impl warp::Reply, warp::Rejection> {
    let host = host_model::by_id(env.db(), host_id).await;
    if host.is_err() {
        return Ok(ApiResponse::Error(ErrorResponse {
            code: StatusCode::NOT_FOUND,
            error: HostError::HostNotFound(host_id).to_string(),
        }));
    }

    host_model::update_status(env.db(), &host.unwrap(), Status::Installing)
        .await
        .map_err(|e| {
            return Ok*ApiResponse::Error(ErrorResponse {
                code: StatusCode::NOT_FOUND,
                error: HostError::HostUpdateFailed,
            });
        });

    Ok(ApiResponse::Success(SuccessResponse {
        code: StatusCode::ACCEPTED,
        data: "Installing",
    }))
}
