#[cfg(test)]
mod test {
    use crate::services::definition::create::create_definition;
    use crate::services::definition::delete::delete_definition;
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
    async fn create_definition_persists_all_fields() {
        let pool = test_support::pool().await;
        let name = format!("test_create_definition_{}", test_support::unique_suffix());

        let created = create_definition(
            &pool,
            NewDefinition {
                name: name.clone(),
                version: "1.0.0".to_string(),
                body: test_body(),
                description: Some("desc".to_string()),
                help: Some("help".to_string()),
            },
        )
        .await
        .expect("create should succeed");

        assert!(created.id > 0);
        assert_eq!(created.name, name);
        assert_eq!(created.version, "1.0.0");
        assert_eq!(created.description, Some("desc".to_string()));
        assert_eq!(created.help, Some("help".to_string()));

        delete_definition(&pool, created.id).await.ok();
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres (docker-compose up)"]
    async fn create_definition_allows_missing_optional_fields() {
        let pool = test_support::pool().await;
        let name = format!("test_create_definition_optional_{}", test_support::unique_suffix());

        let created = create_definition(
            &pool,
            NewDefinition {
                name: name.clone(),
                version: "1.0.0".to_string(),
                body: test_body(),
                description: None,
                help: None,
            },
        )
        .await
        .expect("create should succeed");

        assert_eq!(created.description, None);
        assert_eq!(created.help, None);

        delete_definition(&pool, created.id).await.ok();
    }
}
