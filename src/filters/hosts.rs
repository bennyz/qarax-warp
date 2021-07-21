use super::*;

pub fn hosts(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    list(env.clone()).or(add(env))
}

fn list(
    env: Environment,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(with_env(env))
        .and_then(handlers::hosts::list)
}

fn add(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and(warp::body::json())
        .and(with_env(env))
        .and_then(handlers::hosts::add)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        database,
        models::hosts::{Host, NewHost},
    };
    use dotenv::dotenv;
    use sqlx::PgPool;
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
        // TODO: figure out something better
        sqlx::query("truncate table hosts")
            .execute(pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_add() {
        let pool = setup().await.unwrap();
        let env = Environment::new(pool.clone()).await.unwrap();
        let api = hosts(env.clone());

        let host = NewHost {
            name: String::from("test_host"),
            address: String::from("127.0.0.1"),
            port: 8080,
            host_user: String::from("root"),
            password: String::from("pass"),
        };

        let resp = warp::test::request()
            .method("POST")
            .path("hosts")
            .json(&host)
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
        let resp = warp::test::request()
            .method("GET")
            .path("hosts")
            .reply(&api)
            .await;

        let hosts: HashMap<String, Vec<Host>> =
            serde_json::from_str(std::str::from_utf8(resp.body()).unwrap()).unwrap();

        assert_eq!(hosts.get("response").unwrap().len(), 1);

        teardown(&env.db()).await;
    }
}
