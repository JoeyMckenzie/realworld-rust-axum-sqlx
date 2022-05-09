use sqlx::{Pool, Postgres};
use std::sync::{Arc, Mutex};

pub type SyncAppContext = Arc<AppContext>;

pub struct AppContext {
    pub db: Pool<Postgres>,
}

impl AppContext {
    pub fn new(db: Pool<Postgres>) -> Arc<AppContext> {
        Arc::new(Self { db })
    }
}
