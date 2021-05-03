use std::{fmt, str::FromStr};

use super::*;
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Host {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub port: i32,
    pub status: Status,
    pub host_user: String,

    #[serde(skip_serializing)]
    pub password: String,
}


#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Status {
    Unknown,
    Down,
    Installing,
    Initializing,
    Up,
}

pub async fn add(pool: &PgPool, host: Host) -> anyhow::Result<Uuid> {
    let rec = sqlx::query!(
        r#"
INSERT INTO hosts (name, address, port, status, host_user, password)
VALUES ( $1, $2, $3, $4, $5, $6 )
RETURNING id
"#,
        host.name,
        host.address,
        host.port,
        host.status.to_string(),
        host.host_user,
        host.password
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
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