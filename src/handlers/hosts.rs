use crate::env::Environment;

use super::*;
use models::hosts::HostError;
use models::hosts::NewHost;

pub async fn list(env: Environment) -> Result<impl warp::Reply, warp::Rejection> {
    let hosts = models::hosts::list(env.db()).await;
    match hosts {
        Ok(hosts) => Ok(ApiResponse::Success(SuccessResponse {
            code: StatusCode::OK,
            data: hosts,
        })),
        Err(e) => Ok(ApiResponse::Error(ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            error: HostError::ErrorList(e).to_string(),
        })),
    }
}

pub async fn add(
    host: NewHost,
    env: Environment,
) -> Result<impl warp::Reply, warp::Rejection> {
    let host_id = models::hosts::add(env.db(), &host).await;
    if let Ok(id) = host_id {
        let response = response::Builder::new()
            .status(StatusCode::CREATED)
            .body(json!({ "response": id }).to_string())
            .unwrap();
        Ok(response)
    } else {
        let response = response::Builder::new()
            .status(StatusCode::BAD_REQUEST)
            .body(
                json!({"error": HostError::NameAlreadyExists(host.name.to_owned()).to_string()})
                    .to_string(),
            )
            .unwrap();
        Ok(response)
    }
}
