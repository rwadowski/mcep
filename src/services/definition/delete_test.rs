#[cfg(test)]
mod test {
    use crate::services::definition::create::create_definition;
    use crate::services::definition::delete::delete_definition;
    use crate::services::definition::get::get_definition;
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

    #[tokio::test]
    #[ignore = "requires a reachable Postgres (docker-compose up)"]
    async fn delete_definition_removes_the_row() {
        let pool = test_support::pool().await;
        let name = format!("test_delete_definition_{}", test_support::unique_suffix());
        let created = create_definition(
            &pool,
            NewDefinition {
                name,
                version: "1.0.0".to_string(),
                body: test_body(),
                description: None,
                help: None,
            },
        )
        .await
        .expect("fixture create should succeed");

        let result = delete_definition(&pool, created.id).await;
        assert!(result.is_ok(), "delete should succeed: {result:?}");

        let after = get_definition(&pool, created.id).await;
        assert!(after.is_err(), "definition should no longer be gettable after delete");
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres (docker-compose up)"]
    async fn delete_definition_missing_id_is_a_no_op() {
        let pool = test_support::pool().await;
        let result = delete_definition(&pool, -1).await;
        assert!(result.is_ok(), "deleting a non-existent id should be idempotent: {result:?}");
    }
}
