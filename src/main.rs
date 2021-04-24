mod handlers;
mod filters;


#[tokio::main]
async fn main() {

    warp::serve(filters::hosts::hosts())
        .run(([127, 0, 0, 1], 3030))
        .await;
}