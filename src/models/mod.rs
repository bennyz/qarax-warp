use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use sqlx::sqlx_macros::Type;
use sqlx::types::Uuid;
use std::{fmt, str::FromStr};
use thiserror::Error;

pub mod hosts;
pub mod storage;
