use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use sqlx::query_as;

use conduit_core::users::repository::{UserEntity, UsersRepository};

use crate::connection_pool::ConduitConnectionPool;

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
    async fn search_user_by_email_or_username(
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
        .context("an unexpected error occured while search for users")
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
        .context("an unexpected error occured while creating the user")
    }

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<UserEntity> {
        query_as!(
            UserEntity,
            r#"
        select *
        from users
        where email = $1::varchar
            "#,
            email,
        )
        .fetch_one(self.pool.as_ref())
        .await
        .context("user was not found")
    }

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<UserEntity> {
        query_as!(
            UserEntity,
            r#"
        select *
        from users
        where id = $1
            "#,
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await
        .context("user was not found")
    }
}
