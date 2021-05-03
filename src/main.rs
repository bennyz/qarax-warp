mod models;
mod handlers;
mod filters;

mod env;

#[tokio::main]
async fn main() {
    warp::
    warp::serve(filters::hosts::hosts())
        .run(([127, 0, 0, 1], 3030))
        .await;
}