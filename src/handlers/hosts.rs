use crate::env::Environment;

use super::*;
use models::hosts as host_model;
use models::hosts::{HostError, NewHost, Status};
use uuid::Uuid;

pub async fn list(env: Environment) -> Result<impl warp::Reply, Infallible> {
    let hosts = host_model::list(env.db()).await;
    match hosts {
        Ok(hosts) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(hosts),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::BAD_REQUEST,
            response: QaraxResponse::Error(HostError::ErrorList(e).to_string()),
        }),
    }
}

pub async fn add(host: NewHost, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let host_id = host_model::add(env.db(), &host).await;
    match host_id {
        Ok(host_id) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(host_id),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::BAD_REQUEST,
            response: QaraxResponse::Error(HostError::NameAlreadyExists(host.name).to_string()),
        }),
    }
}

pub async fn install(host_id: Uuid, env: Environment) {
    todo!()
}
