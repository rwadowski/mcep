#![cfg(test)]

use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::runtime::engine::Engine;
use crate::types::config;

/// Connects to the Postgres instance configured in `config/dev.toml`, for use by
/// `#[ignore]`d integration tests. Run with `cargo test -- --ignored` while the
/// dev stack (`docker-compose up`) is running.
pub(crate) async fn pool() -> Pool<Postgres> {
    let cfg = config::load().expect("config should load for tests");
    PgPoolOptions::new()
        .max_connections(2)
        .connect(cfg.database.url().as_str())
        .await
        .expect("test Postgres should be reachable — start the dev stack first")
}

/// Connects to the NATS instance configured in `config/dev.toml` and builds an
/// `Engine`, for `#[ignore]`d integration tests that exercise deploy/undeploy.
pub(crate) async fn engine() -> Arc<Engine> {
    let cfg = config::load().expect("config should load for tests");
    let nats = async_nats::connect(&cfg.nats.host)
        .await
        .expect("test NATS should be reachable — start the dev stack first");
    Engine::new(nats)
}

/// A per-process-unique suffix so concurrent/repeated test runs don't collide on
/// unique columns (e.g. names) or step on each other's rows.
pub(crate) fn unique_suffix() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .to_string()
}
