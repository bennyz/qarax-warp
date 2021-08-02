use crate::models::vms::NewVm;

use super::*;
use models::vms as vm_model;

pub async fn list(env: Environment) -> Result<impl warp::Reply, Infallible> {
    let vms = vm_model::list(env.db()).await;
    match vms {
        Ok(vms) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(vms),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::BAD_REQUEST,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn add(vm: NewVm, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let vm_id = vm_model::add(env.db(), &vm).await;
    match vm_id {
        Ok(vm_id) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(vm_id),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::CONFLICT,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn get(vm_id: Uuid, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let vm = vm_model::by_id(env.db(), vm_id).await;
    match vm {
        Ok(vm) => Ok(ApiResponse {
            code: StatusCode::CREATED,
            response: QaraxResponse::Success(vm),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::NOT_FOUND,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}
