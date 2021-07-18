use super::*;

pub fn hosts(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    list(env.clone()).or(add(env.clone()))
}

fn list(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(with_env(env))
        .and_then(handlers::hosts::list)
}

fn add(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env))
        .and_then(handlers::hosts::add)
}
