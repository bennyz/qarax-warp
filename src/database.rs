use sqlx::{PgPool, migrate::MigrateDatabase, postgres::{self, PgPoolOptions}, sqlx_macros::migrate};

pub async fn connect(db_url: &str) -> anyhow::Result<PgPool> {
    tracing::info!("Initializing database connection");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(db_url: &str, pool: &PgPool) -> anyhow::Result<()> {
    if !postgres::Postgres::database_exists(&db_url).await? {
        tracing::info!("Creating database...");
        postgres::Postgres::create_database(&db_url).await?;
    }

    tracing::info!("Applying migrations");
    Ok(migrate!("./migrations")
    .run(pool)
    .await?)
}