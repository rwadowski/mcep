use log::error;
use sqlx::{Pool, Postgres};
use std::collections::HashSet;

use types::definition::{Definition, DefinitionId};

pub async fn get_definition(pool: &Pool<Postgres>, id: DefinitionId) -> Result<Definition, String> {
    let definition_opt = sqlx::query_as::<_, Definition>("SELECT * FROM definitions WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await;
    match definition_opt {
        Ok(def) => Ok(def),
        Err(err) => {
            error!("{}", err);
            Err(err.to_string())
        }
    }
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
    match definitions_opt {
        Ok(list) => Ok(list),
        Err(err) => {
            error!("{}", err);
            Err(err.to_string())
        }
    }
}
