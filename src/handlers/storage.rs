use super::*;
use models::storage as storage_model;
use models::storage::NewStorage;

pub async fn list(env: Environment) -> Result<impl warp::Reply, Infallible> {
    let storages = storage_model::list(env.db()).await;
    match storages {
        Ok(storages) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(storages),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::BAD_REQUEST,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn add(storage: NewStorage, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let storage_id = storage_model::add(env.db(), &storage).await;
    match storage_id {
        Ok(storage_id) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(storage_id),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::CONFLICT,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn get(storage_id: Uuid, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let storage = storage_model::by_id(env.db(), storage_id).await;
    match storage {
        Ok(storage) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(storage),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::NOT_FOUND,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}
