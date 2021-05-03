use sqlx::PgPool;

use super::*;

pub fn hosts(pool: &sqlx::PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        list(pool)
}

pub fn list(pool: &sqlx::PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("hosts")
        .and(warp::get())
        .and_then(handlers::hosts::list)
}
