use warp::Filter;

use super::handlers;

pub mod hosts;

pub fn qarax() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    hosts::hosts()
}