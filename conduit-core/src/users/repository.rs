use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::{query_as, Error, FromRow, Postgres, Row};
use std::sync::Arc;

pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

pub struct UserEntity {
    pub id: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
}

impl<'a> FromRow<'a, PgRow> for UserEntity {
    fn from_row(row: &'a PgRow) -> Result<Self, Error> {
        Ok(UserEntity {
            id: row.get(0),
            created_at: row.get(1),
            updated_at: row.get(2),
            username: row.get(3),
            email: row.get(4),
            password: row.get(5),
            bio: row.get(6),
            image: row.get(7),
        })
    }
}

#[async_trait]
pub trait UsersRepository {
    async fn get_user_by_email_or_username(
        &self,
        email: String,
        username: String,
    ) -> anyhow::Result<Option<UserEntity>>;
}

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
