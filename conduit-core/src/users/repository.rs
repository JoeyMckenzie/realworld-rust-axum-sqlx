use crate::infrastructure::connection_pool::ConduitConnectionPool;
use crate::users::{UserEntity, UsersRepository};
use anyhow::Context;
use async_trait::async_trait;
use sqlx::query_as;
use std::sync::Arc;

#[derive(Clone)]
pub struct UsersRepositoryImpl {
    pool: Arc<ConduitConnectionPool>,
}

impl UsersRepositoryImpl {
    pub fn new(pool: Arc<ConduitConnectionPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UsersRepository for UsersRepositoryImpl {
    async fn get_user_by_email_or_username(
        &self,
        email: String,
        username: String,
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
}
