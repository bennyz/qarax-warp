use uuid::Uuid;
use warp::Filter;

use crate::env::Environment;

use super::handlers;

pub mod hosts;
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
            .and(storage::storage(env))
            .with(warp::trace::named("storage")))
}

fn with_env(
    env: Environment,
) -> impl Filter<Extract = (Environment,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || env.clone())
}
