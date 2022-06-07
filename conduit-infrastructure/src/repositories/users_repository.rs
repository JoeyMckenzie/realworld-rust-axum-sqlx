use crate::repositories::ConduitConnectionPool;
use async_trait::async_trait;
use conduit_core::users::repository::{UserEntity, UsersRepository};
use sqlx::postgres::PgRow;
use sqlx::{query_as, FromRow, Row};
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
            r#"
        select *
        from users
        where email = $1::varchar
        or username = $2::varchar"#,
            email,
            username
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
