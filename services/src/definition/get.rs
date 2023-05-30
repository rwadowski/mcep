use definition::Definition;
use sqlx::{Pool, Postgres};

pub async fn get_app_definition(pool: &Pool<Postgres>, id: i32) -> Result<Definition, String> {
    let def = sqlx::query_as::<_, Definition>("SELECT * FROM app_definitions WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await;
    Ok(def.unwrap())
}