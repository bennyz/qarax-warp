use sqlx::types::Json;

use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    pub id: Uuid,
    pub name: String,
    pub status: Status,
    pub storage_type: StorageType,
    pub config: Json<StorageConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewStorage {
    pub name: String,
    pub storage_type: StorageType,
    pub config: Json<StorageConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Type, EnumString, Display)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "varchar")]
pub enum StorageType {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "shared")]
    Shared,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Type, EnumString, Display)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "varchar")]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    #[strum(serialize = "up")]
    Up,
    #[strum(serialize = "down")]
    Down,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StorageConfig {
    pub host_id: Option<Uuid>,
    pub path: Option<String>,
    pub pool_name: Option<String>,
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Couldn't list storages: '{0}'")]
    ErrorList(anyhow::Error),
}

pub async fn list(pool: &PgPool) -> anyhow::Result<Vec<Storage>> {
    let storages = sqlx::query_as!(
        Storage,
        r#"
SELECT id, name, status as "status: _", storage_type as "storage_type: _", config as "config: Json<StorageConfig>" 
FROM storage
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| StorageError::ErrorList(anyhow!(e)))?;

    Ok(storages)
}

pub async fn by_id(pool: &PgPool, storage_id: Uuid) -> anyhow::Result<Storage> {
    let storage = sqlx::query_as!(
        Storage,
        r#"
SELECT id, name, status as "status: _", storage_type as "storage_type: _", config as "config: Json<StorageConfig>" 
FROM storage
WHERE id = $1
        "#,
        storage_id
    )
    .fetch_one(pool)
    .await?;

    Ok(storage)
}

pub async fn add(pool: &PgPool, storage: &NewStorage) -> anyhow::Result<Uuid> {
    let rec = sqlx::query!(
        r#"
INSERT INTO storage (name, status, storage_type, config)
VALUES ( $1, $2, $3, $4)
RETURNING id
        "#,
        storage.name,
        &Status::Down.to_string(),
        storage.storage_type.to_string(),
        Json(&storage.config) as _,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}
