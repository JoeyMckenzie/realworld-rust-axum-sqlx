use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use mockall::automock;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::{FromRow, Row};

pub type DynProfilesRepository = Arc<dyn ProfilesRepository + Send + Sync>;

pub struct UserFollowEntity {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub follower_id: i64,
    pub followee_id: i64,
}

#[automock]
#[async_trait]
pub trait ProfilesRepository {
    async fn get_user_followees(&self, user_id: i64) -> anyhow::Result<Vec<UserFollowEntity>>;

    async fn get_user_followers(&self, user_id: i64) -> anyhow::Result<Vec<UserFollowEntity>>;

    async fn add_user_follow(
        &self,
        follower_id: i64,
        followee_id: i64,
    ) -> anyhow::Result<UserFollowEntity>;

    async fn remove_user_follow(&self, follower_id: i64, followee_id: i64) -> anyhow::Result<()>;
}

/// Implements a row/type mapping for sqlx to map our user follow entity directly into a scanned struct from a query.
impl<'a> FromRow<'a, PgRow> for UserFollowEntity {
    fn from_row(row: &'a PgRow) -> Result<Self, sqlx::Error> {
        Ok(UserFollowEntity {
            id: row.get(0),
            created_at: row.get(1),
            follower_id: row.get(2),
            followee_id: row.get(3),
        })
    }
}

impl UserFollowEntity {
    pub fn default() -> Self {
        Self {
            id: 1,
            follower_id: 2,
            followee_id: 1,
            created_at: OffsetDateTime::from(SystemTime::now()),
        }
    }
}
