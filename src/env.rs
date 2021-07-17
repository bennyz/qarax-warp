use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct Environment {
    db_pool: PgPool,
}

impl Environment {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let db_pool = PgPool::connect(database_url).await?;
        Ok(Self { db_pool })
    }

    pub fn db(&self) -> &PgPool {
        &self.db_pool
    }
}
