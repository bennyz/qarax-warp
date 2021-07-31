use std::collections::BTreeMap;

use super::ansible::AnsibleCommand;
use super::*;
use models::hosts as host_model;
use models::hosts::{NewHost, Status};

pub async fn list(env: Environment) -> Result<impl warp::Reply, Infallible> {
    let hosts = host_model::list(env.db()).await;
    match hosts {
        Ok(hosts) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(hosts),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::BAD_REQUEST,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn add(host: NewHost, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let host_id = host_model::add(env.db(), &host).await;
    match host_id {
        Ok(host_id) => Ok(ApiResponse {
            code: StatusCode::CREATED,
            response: QaraxResponse::Success(host_id),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::CONFLICT,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn get(host_id: Uuid, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let host = host_model::by_id(env.db(), host_id).await;
    match host {
        Ok(host) => Ok(ApiResponse {
            code: StatusCode::OK,
            response: QaraxResponse::Success(host),
        }),
        Err(e) => Ok(ApiResponse {
            code: StatusCode::NOT_FOUND,
            response: QaraxResponse::Error(e.to_string()),
        }),
    }
}

pub async fn install(host_id: Uuid, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let host = if let Ok(host) = host_model::by_id(env.db(), host_id).await {
        host
    } else {
        return Ok(ApiResponse {
            code: StatusCode::NOT_FOUND,
            response: QaraxResponse::Error(String::from("Host not found")),
        });
    };

    let updated = host_model::update_status(env.db(), host_id, Status::Installing).await;
    if let Err(e) = updated {
        return Ok(ApiResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            response: QaraxResponse::Error(e.to_string()),
        });
    }

    let mut extra_params = BTreeMap::new();
    extra_params.insert(String::from("ansible_password"), host.password.to_owned());

    // TODO: find a better place for the version
    extra_params.insert(String::from("fcversion"), String::from("0.24.5"));

    tokio::spawn(async move {
        let playbook = AnsibleCommand::new(
            ansible::INSTALL_HOST_PLAYBOOK,
            &host.host_user,
            &host.address,
            extra_params,
        );

        match playbook.run_playbook().await {
            Ok(_) => {
                tracing::info!("Installation successful");
                host_model::update_status(env.db(), host_id, Status::Up)
                    .await
                    .unwrap();
            }

            Err(e) => tracing::error!("Installation failed: {}", e),
        }
    });

    Ok(ApiResponse {
        code: StatusCode::OK,
        response: QaraxResponse::Success("started"),
    })
}
