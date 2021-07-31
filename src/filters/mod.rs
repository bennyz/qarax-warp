use uuid::Uuid;
use warp::Filter;

use crate::env::Environment;

use super::handlers;

pub mod drives;
pub mod hosts;
pub mod kernels;
pub mod storage;

pub fn qarax(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(|| "Hello world!")
        .or(warp::path!("hosts" / ..)
            .and(hosts::hosts(env.clone()))
            .with(warp::trace::named("hosts")))
        .or(warp::path!("storage" / ..)
            .and(storage::storage(env.clone()))
            .with(warp::trace::named("storage")))
        .or(warp::path!("drives" / ..)
            .and(drives::drives(env.clone()))
            .with(warp::trace::named("drives")))
        .or(warp::path!("kernels" / ..)
            .and(kernels::kernels(env))
            .with(warp::trace::named("kernels")))
}

fn with_env(
    env: Environment,
) -> impl Filter<Extract = (Environment,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || env.clone())
}
