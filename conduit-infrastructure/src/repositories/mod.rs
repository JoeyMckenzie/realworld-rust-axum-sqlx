use anyhow::Context;
use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub mod user_repository;

pub type ConduitConnectionPool = Pool<Postgres>;

pub type ArcConduitConnectionPool = Arc<Pool<Postgres>>;

pub async fn initialize_pg_pool(connection_string: &str) -> anyhow::Result<ConduitConnectionPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await
        .context("error while initializing the database connection pool")?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .context("error while running database migrations")?;

    Ok(pool)
}
