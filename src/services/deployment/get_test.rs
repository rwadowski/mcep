#[cfg(test)]
mod test {
    use sqlx::{Pool, Postgres};

    use crate::services::deployment::get::{get_all_deployments, get_deployment};
    use crate::services::test_support;
    use crate::types::deployment::Deployment;

    // create_deployment/delete_deployment require a live Engine (NATS), so fixtures
    // here are inserted directly — get_deployment/get_all_deployments are pure reads.
    async fn insert_test_deployment(pool: &Pool<Postgres>, name: &str) -> Deployment {
        sqlx::query_as::<_, Deployment>(
            "INSERT INTO deployments (name, version, connections, sources, sinks, blocks) \
             VALUES ($1, '1.0.0', '[]', '[]', '[]', '[]') RETURNING *",
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .expect("fixture insert should succeed")
    }

    async fn delete_test_deployment(pool: &Pool<Postgres>, id: i32) {
        sqlx::query("DELETE FROM deployments WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .ok();
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres (docker-compose up)"]
    async fn get_deployment_returns_the_created_row() {
        let pool = test_support::pool().await;
        let name = format!("test_get_deployment_{}", test_support::unique_suffix());
        let created = insert_test_deployment(&pool, &name).await;

        let fetched = get_deployment(&pool, created.id).await.expect("get should succeed");
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, created.name);
        assert_eq!(fetched.version, created.version);

        delete_test_deployment(&pool, created.id).await;
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres (docker-compose up)"]
    async fn get_deployment_missing_id_is_an_error() {
        let pool = test_support::pool().await;
        let result = get_deployment(&pool, -1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires a reachable Postgres (docker-compose up)"]
    async fn get_all_deployments_includes_freshly_created_ones() {
        let pool = test_support::pool().await;
        let name = format!("test_get_all_deployments_{}", test_support::unique_suffix());
        let created = insert_test_deployment(&pool, &name).await;

        let all = get_all_deployments(&pool).await.expect("get_all should succeed");
        assert!(all.iter().any(|d| d.id == created.id));

        delete_test_deployment(&pool, created.id).await;
    }
}
