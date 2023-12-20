use crate::runtime::engine::{EngineActor, EngineActorMessage};
use crate::types::deployment::{Deployment, DeploymentId};
use actix::Addr;
use sqlx::{Error, Pool, Postgres};

pub async fn delete_deployment(
    sender: &Addr<EngineActor>,
    pool: &Pool<Postgres>,
    id: DeploymentId,
) -> Result<(), String> {
    let result: Result<Deployment, Error> =
        sqlx::query_as::<_, Deployment>("DELETE FROM deployments WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await;
    match result {
        Ok(deployment) => {
            sender
                .send(EngineActorMessage::Undeploy(deployment))
                .await
                .expect("TODO: panic message");
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}
