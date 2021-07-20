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
    use crate::{database, models::hosts::NewHost};
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

    async fn teardown() {
        let db_url = &dotenv::var("TEST_DATABASE_URL").expect("DATABASE_URL is not set!");
        postgres::Postgres::drop_database(&db_url).await.unwrap();
    }

    #[tokio::test]
    async fn test_add() {
        let pool = setup().await.unwrap();
        let env = Environment::new(pool).await.unwrap();
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
        drop(api);
        drop(env);
        teardown().await;
    }
}
