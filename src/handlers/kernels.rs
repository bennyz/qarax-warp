use crate::models::kernels::NewKernel;

use super::*;
use models::kernels as kernel_model;

pub async fn list(env: Environment) -> Result<impl warp::Reply, Infallible> {
    let kernels = kernel_model::list(env.db()).await;
    match kernels {
        Ok(kernels) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(kernels),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::BAD_REQUEST,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn add(kernel: NewKernel, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let kernel_id = kernel_model::add(env.db(), &kernel).await;
    match kernel_id {
        Ok(kernel_id) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(kernel_id),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::CONFLICT,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn get(kernel_id: Uuid, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let kernel = kernel_model::by_id(env.db(), kernel_id).await;
    match kernel {
        Ok(kernel) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(kernel),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::NOT_FOUND,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}
