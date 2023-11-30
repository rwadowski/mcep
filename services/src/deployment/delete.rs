use crossbeam_channel::Sender;
use sqlx::{Error, Pool, Postgres};
use types::deployment::{Command, Deployment};

pub async fn delete_deployment(
    sender: &Sender<Command>,
    pool: &Pool<Postgres>,
    id: i32,
) -> Result<(), String> {
    let result: Result<_, Error> =
        sqlx::query_as::<_, Deployment>("DELETE FROM deployment WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
