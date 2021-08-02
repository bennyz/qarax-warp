use super::*;

pub fn vms(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    list(env.clone()).or(get(env.clone())).or(add(env))
}

pub fn list(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(with_env(env))
        .and_then(handlers::vms::list)
}

pub fn get(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!(Uuid)
        .and(warp::get())
        .and(with_env(env))
        .and_then(handlers::vms::get)
}

pub fn add(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env))
        .and_then(handlers::vms::add)
}
