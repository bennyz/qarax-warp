mod filters;
mod handlers;
mod models;

mod database;
mod env;

use tracing_subscriber::fmt::format::FmtSpan;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();
    let db_url = &dotenv::var("DATABASE_URL")
    .expect("DATABASE_URL is not set!");
    let pool = database::connect(&db_url)
        .await?;

    database::run_migrations(&db_url, &pool).await?;
    let environment = env::Environment::new(pool).await?;
    let routes = filters::qarax(environment);

    warp::serve(routes).run(([127, 0, 0, 1], 8888)).await;
    Ok(())
}
