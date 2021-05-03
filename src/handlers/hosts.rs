use super::*;
use models::hosts::Host;

pub async fn list() -> Result<impl warp::Reply, Infallible> {
    let hosts = [];
    Ok(warp::reply::json(r#""#))
}

pub async fn add(pool: &sqlx::PgPool, host: Host) {

}