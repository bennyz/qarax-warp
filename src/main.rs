mod filters;
mod handlers;
mod models;

mod env;

use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let environment = env::Environment::new("postgres://postgres@localhost/qarax").await?;
    let routes = filters::qarax(environment);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8888))
        .await;
    Ok(())
}
