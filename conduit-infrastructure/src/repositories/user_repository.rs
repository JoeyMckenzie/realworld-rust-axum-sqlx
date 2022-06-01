use crate::repositories::ConduitConnectionPool;
use async_trait::async_trait;
use conduit_core::users::repository::UsersRepository;
use conduit_domain::users::models::User;
use sqlx::{query, Pool, Postgres};
use std::sync::Arc;

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
    ) -> anyhow::Result<User> {
        let existing_user = query!(
            r#"
        select *
        from users
        where email = $1::varchar
        or username = $2::varchar"#,
            email,
            username
        )
        .fetch_optional(self.pool.as_ref())
        .await;

        Ok(User {
            id: 1_u64,
            created_at: Default::default(),
            updated_at: Default::default(),
            username: "".to_string(),
            email: String::from("email"),
            password: String::from("password"),
        })
    }
}
