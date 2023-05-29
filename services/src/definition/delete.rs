use rocket::data::ToByteUnit;
use sqlx::{Error, Pool, Postgres};
use definition::Definition;

pub async fn delete_definition(pool: &Pool<Postgres>, id: i32) -> Result<(), String> {
    let r: Result<_, Error> = sqlx::query_as::<_, Definition>("DELETE FROM app_definitions WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await;
    Ok(())
}