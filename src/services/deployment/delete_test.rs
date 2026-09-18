#[cfg(test)]
mod test {
    use crate::services::deployment::create::create_deployment;
    use crate::services::deployment::delete::delete_deployment;
    use crate::services::deployment::get::get_deployment;
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
    async fn delete_deployment_removes_the_row_and_undeploys() {
        let pool = test_support::pool().await;
        let engine = test_support::engine().await;
        let name = format!("test_delete_deployment_{}", test_support::unique_suffix());

        let created = create_deployment(&engine, &pool, empty_deployment(name))
            .await
            .expect("fixture create should succeed");

        let result = delete_deployment(&engine, &pool, created.id).await;
        assert!(result.is_ok(), "delete should succeed: {result:?}");

        let after = get_deployment(&pool, created.id).await;
        assert!(after.is_err(), "deployment should no longer be gettable after delete");
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres + NATS (docker-compose up)"]
    async fn delete_deployment_missing_id_is_an_error() {
        let pool = test_support::pool().await;
        let engine = test_support::engine().await;
        let result = delete_deployment(&engine, &pool, -1).await;
        assert!(result.is_err());
    }
}
