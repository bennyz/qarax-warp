mod filters;
mod handlers;
mod models;

mod database;
mod env;

use dotenv::dotenv;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "qarax=info,warp=trace".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db_url = &dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set!");
    database::run_migrations(&db_url).await?;
    let pool = database::connect(&db_url).await?;

    let environment = env::Environment::new(pool).await?;
    let routes = filters::qarax(environment);

    warp::serve(routes).run(([127, 0, 0, 1], 8888)).await;
    Ok(())
}
