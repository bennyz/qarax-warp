use crate::models::drives::NewDrive;

use super::*;
use models::drives as drive_model;

pub async fn list(env: Environment) -> Result<impl warp::Reply, Infallible> {
    let drives = drive_model::list(env.db()).await;
    match drives {
        Ok(drives) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(drives),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::BAD_REQUEST,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn add(drive: NewDrive, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let drive_id = drive_model::add(env.db(), &drive).await;
    match drive_id {
        Ok(drive_id) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(drive_id),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::CONFLICT,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn get(drive_id: Uuid, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let drive = drive_model::by_id(env.db(), drive_id).await;
    match drive {
        Ok(drive) => Ok(ApiResponse {
            code: StatusCode::CREATED,
            response: QaraxResponse::Success(drive),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::NOT_FOUND,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}
