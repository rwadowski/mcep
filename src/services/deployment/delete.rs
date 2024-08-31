use crate::runtime::engine::{EngineActor, EngineActorMessage};
use crate::services::deployment::get;
use crate::types::deployment::{Deployment, DeploymentId};
use actix::Addr;
use sqlx::{Error, Pool, Postgres};

pub async fn delete_deployment(
    sender: &Addr<EngineActor>,
    pool: &Pool<Postgres>,
    id: DeploymentId,
) -> Result<(), String> {
    let result = get::get_deployment(&pool, id).await;
    match result {
        Ok(deployment) => {
            sender
                .send(EngineActorMessage::Undeploy(deployment))
                .await
                .expect("TODO: panic message");
            let delete_result: Result<(), Error> =
                sqlx::query_as::<_, _>("DELETE FROM deployments WHERE id = $1")
                    .bind(id)
                    .fetch_one(pool)
                    .await;
            delete_result.map_err(|err| err.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}
