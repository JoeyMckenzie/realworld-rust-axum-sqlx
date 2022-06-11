use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use sqlx::query_as;

use conduit_core::profiles::repository::{ProfilesRepository, UserFollowEntity};

use crate::connection_pool::ConduitConnectionPool;

#[derive(Clone)]
pub struct PostgresProfilesRepository {
    pool: Arc<ConduitConnectionPool>,
}

impl PostgresProfilesRepository {
    pub fn new(pool: Arc<ConduitConnectionPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfilesRepository for PostgresProfilesRepository {
    async fn get_user_followees(&self, user_id: i64) -> anyhow::Result<Vec<UserFollowEntity>> {
        query_as!(
            UserFollowEntity,
            r#"
        select id,
               created_at,
               follower_id,
               followee_id
        from user_follows
        where followee_id = $1"#,
            user_id
        )
        .fetch_all(self.pool.as_ref())
        .await
        .context("an unexpected error occured retrieving user follows")
    }
}
