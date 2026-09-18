#[cfg(test)]
mod test {
    use crate::services::deployment::create::create_deployment;
    use crate::services::deployment::delete::delete_deployment;
    use crate::services::test_support;
    use crate::types::deployment::NewDeployment;

    fn empty_deployment(name: String) -> NewDeployment {
        NewDeployment {
            name,
            version: "1.0.0".to_string(),
            connections: vec![],
            sources: vec![],
            sinks: vec![],
            blocks: vec![],
        }
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres + NATS (docker-compose up)"]
    async fn create_deployment_persists_and_deploys_an_empty_flow() {
        let pool = test_support::pool().await;
        let engine = test_support::engine().await;
        let name = format!("test_create_deployment_{}", test_support::unique_suffix());

        let created = create_deployment(&engine, &pool, empty_deployment(name.clone()))
            .await
            .expect("create should succeed");

        assert!(created.id > 0);
        assert_eq!(created.name, name);
        assert_eq!(created.version, "1.0.0");
        assert!(created.blocks.is_empty());
        assert!(created.sources.is_empty());
        assert!(created.sinks.is_empty());

        delete_deployment(&engine, &pool, created.id).await.ok();
    }
}
