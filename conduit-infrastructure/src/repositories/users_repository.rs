use crate::connection_pool::ConduitConnectionPool;
use anyhow::Context;
use async_trait::async_trait;
use conduit_core::users::repository::{UserEntity, UsersRepository};
use sqlx::query_as;
use std::sync::Arc;

#[derive(Clone)]
pub struct PostgresUsersRepository {
    pool: Arc<ConduitConnectionPool>,
}

impl PostgresUsersRepository {
    pub fn new(pool: Arc<ConduitConnectionPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UsersRepository for PostgresUsersRepository {
    async fn get_user_by_email_or_username(
        &self,
        email: &str,
        username: &str,
    ) -> anyhow::Result<Option<UserEntity>> {
        query_as!(
            UserEntity,
            r#"
        select id,
               created_at,
               updated_at,
               username,
               email,
               password,
               bio,
               image
        from users
        where email = $1::varchar
        or username = $2::varchar"#,
            email,
            username
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .context("an unexpected error occured while retrieving the user")
    }

    async fn create_user(
        &self,
        email: &str,
        username: &str,
        hashed_password: &str,
    ) -> anyhow::Result<UserEntity> {
        query_as!(
            UserEntity,
            r#"
        insert into users (created_at, updated_at, username, email, password, bio, image)
        values (current_timestamp, current_timestamp, $1::varchar, $2::varchar, $3::varchar, '', '')
        returning *
            "#,
            username,
            email,
            hashed_password
        )
        .fetch_one(self.pool.as_ref())
        .await
        .context("an unexpected error occured while retrieving the user")
    }
}
