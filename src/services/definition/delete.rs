use sqlx::{Pool, Postgres};

pub async fn delete_definition(pool: &Pool<Postgres>, id: i32) -> Result<(), String> {
    sqlx::query("DELETE FROM definitions WHERE id = $1 RETURNING id")
        .bind(id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}
