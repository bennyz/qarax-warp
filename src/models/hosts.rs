use std::{fmt, str::FromStr};

use super::*;
use sqlx::postgres::PgPool;
use sqlx::sqlx_macros::Type;
use sqlx::types::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Host {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub port: i32,
    pub status: Status,
    pub host_user: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewHost {
    pub name: String,
    pub address: String,
    pub port: i32,
    pub host_user: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Type)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "varchar")]
pub enum Status {
    Unknown,
    Down,
    Installing,
    Initializing,
    Up,
}

#[derive(Error, Debug)]
pub enum HostError {
    #[error("Host with name '{0}' already exists")]
    NameAlreadyExists(String),
    #[error("Couldn't list hosts: '{0}'")]
    ErrorList(anyhow::Error),
    #[error("Host not found: '{0}'")]
    HostNotFound(Uuid),
}

pub async fn list(pool: &PgPool) -> anyhow::Result<Vec<Host>> {
    let hosts = sqlx::query_as!(
        Host,
        r#"
SELECT id, name, address, port, status as "status: _", host_user FROM hosts
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(hosts)
}

pub async fn add(pool: &PgPool, host: &NewHost) -> anyhow::Result<Uuid> {
    let rec = sqlx::query!(
        r#"
INSERT INTO hosts (name, address, port, status, host_user, password)
VALUES ( $1, $2, $3, $4, $5, $6 )
RETURNING id
"#,
        host.name,
        host.address,
        host.port,
        &Status::Down.to_string(),
        host.host_user,
        host.password
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn by_id(pool: &PgPool, host_id: Uuid) -> anyhow::Result<Host> {
    let host = sqlx::query_as!(
        Host,
        r#"
SELECT id, name, address, port, status as "status: _", host_user
FROM hosts
WHERE id = $1
        "#,
        host_id
    )
    .fetch_one(pool)
    .await?;

    Ok(host)
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Down => write!(f, "down"),
            Status::Initializing => write!(f, "initializing"),
            Status::Installing => write!(f, "installing"),
            Status::Unknown => write!(f, "unknown"),
            Status::Up => write!(f, "up"),
        }
    }
}

impl FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "down" => Ok(Status::Down),
            "initializing" => Ok(Status::Initializing),
            "installing" => Ok(Status::Installing),
            "unknown" => Ok(Status::Unknown),
            "up" => Ok(Status::Initializing),
            _ => anyhow::bail!("No matching variant!"),
        }
    }
}
