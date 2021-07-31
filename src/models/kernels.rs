use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kernel {
    pub id: Uuid,
    pub name: String,
    pub storage_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewKernel {
    pub name: String,
    pub storage_id: Uuid,
}

pub async fn list(pool: &PgPool) -> anyhow::Result<Vec<Kernel>> {
    let kernels = sqlx::query_as!(
        Kernel,
        r#"
SELECT id, name, storage_id
FROM kernels
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(kernels)
}

pub async fn by_id(pool: &PgPool, kernel_id: Uuid) -> anyhow::Result<Kernel> {
    let kernel = sqlx::query_as!(
        Kernel,
        r#"
SELECT id, name, storage_id
FROM kernels
WHERE id = $1
        "#,
        kernel_id
    )
    .fetch_one(pool)
    .await?;

    Ok(kernel)
}

pub async fn add(pool: &PgPool, kernel: &NewKernel) -> anyhow::Result<Uuid> {
    let rec = sqlx::query!(
        r#"
INSERT INTO kernels (name, storage_id)
VALUES ( $1, $2)
RETURNING id
        "#,
        kernel.name,
        kernel.storage_id
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}
