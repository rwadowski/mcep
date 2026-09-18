use crate::types::definition::{Definition, UpdateDefinition};
use log::{error, info};
use sqlx::{Error, Pool, Postgres, QueryBuilder};

pub async fn update_definition<'a>(
    pool: &Pool<Postgres>,
    update: UpdateDefinition,
) -> Result<Definition, String> {
    let mut builder = new_builder(&update);
    let query = builder.build_query_as::<Definition>();
    let result: Result<Definition, Error> = query.fetch_one(pool).await;
    match result {
        Ok(definition) => {
            info!("definition {} updated", definition.id);
            Ok(definition)
        }
        Err(err) => {
            error!("definition {} updated failed - {}", update.id, err);
            Err(err.to_string())
        }
    }
}
pub(crate) fn new_builder(definition: &UpdateDefinition) -> QueryBuilder<Postgres> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE definitions SET ");
    let mut sep = query_builder.separated(", ");
    let mut has_fields = false;
    if let Some(version) = &definition.version {
        sep.push("version = ")
            .push_bind_unseparated(version.clone());
        has_fields = true;
    }
    if let Some(name) = &definition.name {
        sep.push("name = ").push_bind_unseparated(name.clone());
        has_fields = true;
    }
    if let Some(body) = &definition.body {
        sep.push("body = ").push_bind_unseparated(body.clone());
        has_fields = true;
    }
    if let Some(body_type) = &definition.body_type {
        sep.push("body_type = ")
            .push_bind_unseparated(body_type.clone());
        has_fields = true;
    }
    if let Some(description) = &definition.description {
        sep.push("description = ")
            .push_bind_unseparated(description.clone());
        has_fields = true;
    }
    if let Some(help) = &definition.help {
        sep.push("help = ").push_bind_unseparated(help.clone());
        has_fields = true;
    }
    query_builder.push(if has_fields {
        " WHERE id = "
    } else {
        "WHERE id = "
    });
    query_builder.push_bind(definition.id);
    query_builder.push(" RETURNING *");
    query_builder
}
