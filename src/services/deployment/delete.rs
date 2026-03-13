use sqlx::{Pool, Postgres};

use crate::runtime::engine::Engine;
use crate::services::deployment::get;

pub async fn delete_deployment(
    engine: &Engine,
    pool: &Pool<Postgres>,
    id: i32,
) -> Result<(), String> {
    let deployment = get::get_deployment(pool, id)
        .await
        .map_err(|e| e.to_string())?;
    engine.undeploy(&deployment).await;
    sqlx::query("DELETE FROM deployments WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}
