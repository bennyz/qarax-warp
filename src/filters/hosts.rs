use super::*;

pub fn hosts() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        list()
}

pub fn list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("hosts")
        .and(warp::get())
        .and_then(handlers::hosts::list)
}
