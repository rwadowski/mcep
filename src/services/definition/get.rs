use sqlx::{Pool, Postgres};
use std::collections::HashSet;

use crate::types::definition::{Definition, DefinitionId};
use crate::utils;

pub async fn get_definition(pool: &Pool<Postgres>, id: DefinitionId) -> Result<Definition, String> {
    let definition_opt = sqlx::query_as::<_, Definition>("SELECT * FROM definitions WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await;
    definition_opt.map_err(utils::log_and_convert_to_string)
}

pub async fn get_definitions(
    pool: &Pool<Postgres>,
    ids: HashSet<DefinitionId>,
) -> Result<Vec<Definition>, String> {
    let list: Vec<DefinitionId> = ids.into_iter().collect();
    let definitions_opt =
        sqlx::query_as::<_, Definition>("SELECT * FROM definitions WHERE id = ANY($1)")
            .bind(list)
            .fetch_all(pool)
            .await;
    definitions_opt.map_err(utils::log_and_convert_to_string)
}

pub async fn get_all_definitions(pool: &Pool<Postgres>) -> Result<Vec<Definition>, String> {
    let definitions_opt = sqlx::query_as::<_, Definition>("SELECT * FROM definitions")
        .fetch_all(pool)
        .await;
    definitions_opt.map_err(utils::log_and_convert_to_string)
}
