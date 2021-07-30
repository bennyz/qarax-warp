use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Drive {
    pub id: Uuid,
    pub name: String,
    pub status: Status,
    pub readonly: bool,
    pub rootfs: bool,
    pub storage_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewDrive {
    pub name: String,
    pub readonly: bool,
    pub rootfs: bool,
    pub storage_id: Uuid,
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

pub async fn list(pool: &PgPool) -> anyhow::Result<Vec<Drive>> {
    let drives = sqlx::query_as!(
        Drive,
        r#"
SELECT id, name, status as "status: _", readonly, rootfs, storage_id
FROM drives
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(drives)
}

pub async fn by_id(pool: &PgPool, drive_id: Uuid) -> anyhow::Result<Drive> {
    let drive = sqlx::query_as!(
        Drive,
        r#"
SELECT id, name, status as "status: _", readonly, rootfs, storage_id 
FROM drives
WHERE id = $1
        "#,
        drive_id
    )
    .fetch_one(pool)
    .await?;

    Ok(drive)
}

pub async fn add(pool: &PgPool, drive: &NewDrive) -> anyhow::Result<Uuid> {
    let rec = sqlx::query!(
        r#"
INSERT INTO drives (name, status, readonly, rootfs, storage_id)
VALUES ( $1, $2, $3, $4, $5)
RETURNING id
        "#,
        drive.name,
        &Status::Down.to_string(),
        drive.readonly,
        drive.rootfs,
        drive.storage_id
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}
