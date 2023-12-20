use crate::types::deployment::Deployment;
use sqlx::{Pool, Postgres};

pub async fn get_deployment(pool: &Pool<Postgres>, id: i32) -> Result<Deployment, String> {
    let deployment_opt = sqlx::query_as::<_, Deployment>("SELECT * FROM deployments WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await;
    match deployment_opt {
        Ok(dep) => Ok(dep),
        Err(err) => Err(err.to_string()),
    }
}
