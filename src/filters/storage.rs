use super::*;

pub fn storage(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    list(env.clone()).or(get(env.clone())).or(add(env))
}

pub fn list(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(with_env(env))
        .and_then(handlers::storage::list)
}

pub fn get(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!(Uuid)
        .and(warp::get())
        .and(with_env(env))
        .and_then(handlers::storage::get)
}

pub fn add(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env))
        .and_then(handlers::storage::add)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        database,
        models::storage::{NewStorage, Storage, StorageConfig, StorageType::Local},
    };

    use dotenv::dotenv;
    use sqlx::{migrate::MigrateDatabase, postgres, PgPool};
    use warp::hyper::StatusCode;

    use super::*;

    async fn setup() -> anyhow::Result<PgPool> {
        dotenv().ok();
        let db_url = &dotenv::var("TEST_DATABASE_URL").expect("DATABASE_URL is not set!");
        database::run_migrations(db_url).await.unwrap();
        let pool = database::connect(&db_url).await?;

        Ok(pool)
    }

    async fn teardown(pool: &PgPool) {
        pool.close().await;
        let db_url = &dotenv::var("TEST_DATABASE_URL").expect("DATABASE_URL is not set!");
        postgres::Postgres::drop_database(db_url).await.unwrap();
    }

    #[tokio::test]
    async fn test_add() {
        let pool = setup().await.unwrap();
        let env = Environment::new(pool.clone()).await.unwrap();
        let api = storage(env.clone());
        let config = StorageConfig {
            host_id: None,
            path: Some(String::from("/root")),
            pool_name: Some(String::from("volumes")),
        };

        let storage = NewStorage {
            name: String::from("test_storage"),
            storage_type: Local,
            config,
        };

        let resp = warp::test::request()
            .method("POST")
            .path("storage")
            .json(&storage)
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
        let resp = warp::test::request()
            .method("GET")
            .path("storage")
            .reply(&api)
            .await;

        let storages: HashMap<String, Vec<Storage>> =
            serde_json::from_str(std::str::from_utf8(resp.body()).unwrap()).unwrap();

        assert_eq!(storages.get("response").unwrap().len(), 1);

        teardown(&env.db()).await;
    }
}
