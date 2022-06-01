use crate::config::AppConfig;
use sqlx::PgPool;
use std::sync::Arc;

pub type SyncAppContext = Arc<AppContext>;

pub struct AppContext {
    pub pool: PgPool,
    pub config: AppConfig,
}

impl AppContext {
    pub fn new(pool: PgPool, config: AppConfig) -> Arc<AppContext> {
        Arc::new(Self { pool, config })
    }
}
