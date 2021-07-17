mod filters;
mod handlers;
mod models;

mod env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let environment = env::Environment::new("postgres://postgres@localhost/qarax").await?;
    warp::serve(filters::qarax(environment))
        .run(([127, 0, 0, 1], 8888))
        .await;
    Ok(())
}
