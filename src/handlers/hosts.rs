use crate::env::Environment;

use super::*;
use models::hosts::NewHost;

pub async fn list(env: Environment) -> anyhow::Result<impl warp::Reply, warp::Rejection> {
    let hosts = models::hosts::list(env.db()).await;
    match hosts {
        Ok(hosts) => {
            let response = response::Builder::new()
                .status(StatusCode::CREATED)
                .body(json!({ "response": hosts }).to_string())
                .unwrap();
            Ok(response)
        }
        Err(e) => {
            let response = response::Builder::new()
                .status(StatusCode::BAD_REQUEST)
                .body(
                    json!({"error": models::hosts::HostError::ErrorList(e.to_string())})
                        .to_string(),
                )
                .unwrap();
            Ok(response)
        }
    }
}

pub async fn add(
    host: NewHost,
    env: Environment,
) -> anyhow::Result<impl warp::Reply, warp::Rejection> {
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
                json!({"error": models::hosts::HostError::NameAlreadyExists(host.name.to_owned())})
                    .to_string(),
            )
            .unwrap();
        Ok(response)
    }
}
