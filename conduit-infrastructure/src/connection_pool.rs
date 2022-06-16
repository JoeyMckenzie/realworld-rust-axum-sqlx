use std::sync::Arc;
use anyhow::Context;
use sqlx::{Pool, Postgres, Transaction};
use sqlx::postgres::PgPoolOptions;
use tracing::info;

pub type ConduitConnectionPool = Pool<Postgres>;

pub type ConduitConnectionTransaction = Arc<Transaction<'static, Postgres>>;

pub struct ConduitConnectionManager;

impl ConduitConnectionManager {
    pub async fn new_pg_pool(
        connection_string: &str,
        run_migrations: bool,
    ) -> anyhow::Result<ConduitConnectionPool> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .context("error while initializing the database connection pool")?;

        if run_migrations {
            info!("migrations enabled, running...");
            sqlx::migrate!()
                .run(&pool)
                .await
                .context("error while running database migrations")?;
        }

        Ok(pool)
    }
}
