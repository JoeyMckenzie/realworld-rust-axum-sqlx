use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};

use conduit_core::profiles::repository::{ProfilesRepository, UserFollowEntity};

use crate::connection_pool::ConduitConnectionPool;

#[derive(Clone)]
pub struct PostgresProfilesRepository {
    pool: ConduitConnectionPool,
}

impl PostgresProfilesRepository {
    pub fn new(pool: ConduitConnectionPool) -> Self {
        Self { pool: pool }
    }
}

#[async_trait]
impl ProfilesRepository for PostgresProfilesRepository {
    async fn get_user_followees(&self, user_id: i64) -> anyhow::Result<Vec<UserFollowEntity>> {
        // search for users whose followers match the user_id to get the list of user followees
        query_as!(
            UserFollowEntity,
            r#"
        select id,
               created_at,
               follower_id,
               followee_id
        from user_follows
        where follower_id = $1"#,
            user_id
        )
            .fetch_all(&self.pool)
            .await
            .context("an unexpected error occured retrieving user followees")
    }

    async fn get_user_followers(&self, user_id: i64) -> anyhow::Result<Vec<UserFollowEntity>> {
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
            .fetch_all(&self.pool)
            .await
            .context("an unexpected error occured retrieving user followers")
    }

    async fn add_user_follow(
        &self,
        follower_id: i64,
        followee_id: i64,
    ) -> anyhow::Result<UserFollowEntity> {
        query_as!(
            UserFollowEntity,
            r#"
        insert into user_follows (created_at, follower_id, followee_id)
        values (current_timestamp, $1, $2)
        returning *"#,
            follower_id,
            followee_id
        )
            .fetch_one(&self.pool)
            .await
            .context("an unexpected error occured retrieving user follows")
    }

    #[allow(unused_must_use)]
    async fn remove_user_follow(&self, follower_id: i64, followee_id: i64) -> anyhow::Result<()> {
        query!(
            r#"
        delete from user_follows
        where (follower_id, followee_id) = ($1, $2)
            "#,
            follower_id,
            followee_id
        )
            .execute(&self.pool)
            .await
            .context("an unexpected error occured retrieving user follows");

        Ok(())
    }
}
