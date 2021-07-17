use crate::env::Environment;

use super::*;
use models::hosts::NewHost;

pub async fn list(env: Environment) -> Result<impl warp::Reply, Infallible> {
    let hosts = models::hosts::list(env.db()).await;
    match hosts {
        Ok(hosts) => Ok(warp::reply::json(&hosts)),
        Err(e) => Ok(warp::reply::json(&models::hosts::HostError::ErrorList(
            e.to_string(),
        ))),
    }
}

pub async fn add(host: NewHost, env: Environment) -> Result<impl warp::Reply, Infallible> {
    let host_id = models::hosts::add(env.db(), &host).await;
    if let Ok(id) = host_id {
        Ok(warp::reply::json(&id))
    } else {
        Ok(warp::reply::json(
            &models::hosts::HostError::NameAlreadyExists(host.name.to_owned()),
        ))
    }
}
