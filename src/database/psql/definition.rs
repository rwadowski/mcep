use crate::database::DefinitionStore;
use crate::types::definition::{Definition, DefinitionId, NewDefinition, UpdateDefinition};
use crate::utils;
use log::{error, info};
use sqlx::{Pool, Postgres, QueryBuilder};

pub struct PsqlDefinitionStore {
    pool: Pool<Postgres>,
}

impl Clone for PsqlDefinitionStore {
    fn clone(&self) -> Self {
        PsqlDefinitionStore {
            pool: self.pool.clone(),
        }
    }
}
impl DefinitionStore for PsqlDefinitionStore {
    async fn create_definition(&self, definition: &NewDefinition) -> Result<Definition, String> {
        let mut builder = insert_builder(&definition)?;
        let query = builder.build_query_as::<Definition>();
        let result = query.fetch_one(&self.pool).await;
        match result {
            Ok(created_def) => {
                info!("definition {} created", created_def.id.to_string());
                Ok(created_def)
            }
            Err(err) => {
                error!("{}", err.to_string());
                Err(err.to_string())
            }
        }
    }

    async fn update_definition(&self, update: &UpdateDefinition) -> Result<Definition, String> {
        let mut builder = update_builder(&update)?;
        let query = builder.build_query_as::<Definition>();
        let result = query.fetch_one(&self.pool).await;
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

    async fn get_definition(&self, id: DefinitionId) -> Result<Definition, String> {
        let mut builder = get_builder(&[id])?;
        let query = builder.build_query_as::<Definition>();
        let result = query.fetch_one(&self.pool).await;
        result.map_err(utils::log_and_convert_to_string)
    }

    async fn get_definitions(&self, ids: &Vec<DefinitionId>) -> Result<Vec<Definition>, String> {
        let mut builder = get_builder(ids.as_slice())?;
        let query = builder.build_query_as::<Definition>();
        let result = query.fetch_all(&self.pool).await;
        result.map_err(utils::log_and_convert_to_string)
    }

    async fn get_all_definitions(&self) -> Result<Vec<Definition>, String> {
        let mut builder = get_builder(&[])?;
        let query = builder.build_query_as::<Definition>();
        let result = query.fetch_all(&self.pool).await;
        result.map_err(utils::log_and_convert_to_string)
    }

    async fn delete_definition(&self, id: DefinitionId) -> Result<(), String> {
        let mut builder = delete_builder(id)?;
        let query = builder.build();
        let result = query.fetch_one(&self.pool).await;
        result.map(|_| ()).map_err(utils::log_and_convert_to_string)
    }
}

impl PsqlDefinitionStore {
    pub fn new(pool: Pool<Postgres>) -> PsqlDefinitionStore {
        PsqlDefinitionStore { pool }
    }
}

fn insert_builder(new_def: &NewDefinition) -> Result<QueryBuilder<Postgres>, String> {
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO definitions (name, version, body, description, help)");
    let body = new_def.body.as_json()?;
    query_builder.push_values([new_def], |mut b, def| {
        b.push_bind(def.name.clone())
            .push_bind(new_def.version.clone())
            .push_bind(body.clone())
            .push_bind(new_def.description.clone())
            .push_bind(def.help.clone());
    });

    query_builder.push(" RETURNING *");
    Ok(query_builder)
}
fn update_builder(definition: &UpdateDefinition) -> Result<QueryBuilder<Postgres>, String> {
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
    Ok(query_builder)
}

fn get_builder(ids: &[DefinitionId]) -> Result<QueryBuilder<Postgres>, String> {
    let builder = if ids.is_empty() {
        QueryBuilder::new("SELECT * FROM definitions")
    } else {
        let mut b = QueryBuilder::new("SELECT * FROM definitions WHERE id = ANY(");
        b.push_bind(ids.to_vec()).push(")");
        b
    };
    Ok(builder)
}

fn delete_builder(id: DefinitionId) -> Result<QueryBuilder<Postgres>, String> {
    let mut builder = QueryBuilder::new("DELETE FROM definitions WHERE id = ");
    builder.push_bind(id);
    builder.push(" RETURNING id");
    Ok(builder)
}
