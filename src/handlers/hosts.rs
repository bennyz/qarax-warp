use  super::*;
#[derive(Debug, Serialize)]
struct Host {
    pub host_id: Uuid,
}

pub async fn list() -> Result<impl warp::Reply, Infallible> {
    let hosts = [Host{host_id: Uuid::new_v4()}];
    Ok(warp::reply::json(&hosts))
}
