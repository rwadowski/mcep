use crate::types::deployment::Deployment;
use crate::utils;
use sqlx::{Pool, Postgres};

pub async fn get_deployment(pool: &Pool<Postgres>, id: i32) -> Result<Deployment, String> {
    let deployment_opt = sqlx::query_as::<_, Deployment>("SELECT * FROM deployments WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await;
    deployment_opt.map_err(utils::log_and_convert_to_string)
}

pub async fn get_all_deployments(pool: &Pool<Postgres>) -> Result<Vec<Deployment>, String> {
    let deployments_opt = sqlx::query_as::<_, Deployment>("SELECT * FROM deployments")
        .fetch_all(pool)
        .await;
    deployments_opt.map_err(utils::log_and_convert_to_string)
}
