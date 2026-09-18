#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::services::definition::create::create_definition;
    use crate::services::definition::delete::delete_definition;
    use crate::services::definition::get::{get_all_definitions, get_definition, get_definitions};
    use crate::services::test_support;
    use crate::types::definition::block::code::CodeBlock;
    use crate::types::definition::block::Block;
    use crate::types::definition::NewDefinition;

    fn test_body() -> Box<dyn Block> {
        Box::new(CodeBlock {
            inputs: vec![],
            outputs: vec![],
            source: "function f(x){return x}".to_string(),
            dependencies: vec![],
        })
    }

    async fn create_test_definition(pool: &sqlx::Pool<sqlx::Postgres>, name: &str) -> crate::types::definition::Definition {
        create_definition(
            pool,
            NewDefinition {
                name: name.to_string(),
                version: "1.0.0".to_string(),
                body: test_body(),
                description: None,
                help: None,
            },
        )
        .await
        .expect("fixture create should succeed")
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres (docker-compose up)"]
    async fn get_definition_returns_the_created_row() {
        let pool = test_support::pool().await;
        let name = format!("test_get_definition_{}", test_support::unique_suffix());
        let created = create_test_definition(&pool, &name).await;

        let fetched = get_definition(&pool, created.id).await.expect("get should succeed");
        assert_eq!(fetched, created);

        delete_definition(&pool, created.id).await.ok();
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres (docker-compose up)"]
    async fn get_definition_missing_id_is_an_error() {
        let pool = test_support::pool().await;
        let result = get_definition(&pool, -1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres (docker-compose up)"]
    async fn get_definitions_returns_exactly_the_requested_ids() {
        let pool = test_support::pool().await;
        let suffix = test_support::unique_suffix();
        let a = create_test_definition(&pool, &format!("test_get_definitions_a_{suffix}")).await;
        let b = create_test_definition(&pool, &format!("test_get_definitions_b_{suffix}")).await;
        let c = create_test_definition(&pool, &format!("test_get_definitions_c_{suffix}")).await;

        let ids: HashSet<i32> = HashSet::from([a.id, b.id]);
        let fetched = get_definitions(&pool, ids).await.expect("get should succeed");
        let fetched_ids: HashSet<i32> = fetched.iter().map(|d| d.id).collect();

        assert_eq!(fetched_ids, HashSet::from([a.id, b.id]));
        assert!(!fetched_ids.contains(&c.id));

        delete_definition(&pool, a.id).await.ok();
        delete_definition(&pool, b.id).await.ok();
        delete_definition(&pool, c.id).await.ok();
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres (docker-compose up)"]
    async fn get_all_definitions_includes_freshly_created_ones() {
        let pool = test_support::pool().await;
        let name = format!("test_get_all_definitions_{}", test_support::unique_suffix());
        let created = create_test_definition(&pool, &name).await;

        let all = get_all_definitions(&pool).await.expect("get_all should succeed");
        assert!(all.iter().any(|d| d.id == created.id));

        delete_definition(&pool, created.id).await.ok();
    }
}
