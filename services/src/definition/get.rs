use sqlx::{Pool, Postgres};
use types::definition::Definition;

pub async fn get_app_definition(pool: &Pool<Postgres>, id: i32) -> Result<Definition, String> {
    let definition_opt = sqlx::query_as::<_, Definition>("SELECT * FROM definitions WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await;
    match definition_opt {
        Ok(def) => Ok(def),
        Err(err) => Err(err.to_string()),
    }
}