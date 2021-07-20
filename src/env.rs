use sqlx::postgres::PgPool;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct Environment {
    pool: PgPool,
}

impl Environment {
    pub async fn new(pool: PgPool) -> anyhow::Result<Self> {
        Ok(Self { pool })
    }

    pub fn db(&self) -> &PgPool {
        &self.pool
    }
}
