use async_trait::async_trait;
use conduit_domain::users::models::UserDto;
use sqlx::postgres::PgRow;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::{FromRow, Row};
use std::sync::Arc;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[async_trait]
pub trait UsersRepository {
    async fn get_user_by_email_or_username(
        &self,
        email: &str,
        username: &str,
    ) -> anyhow::Result<Option<UserEntity>>;

    async fn create_user(
        &self,
        email: &str,
        username: &str,
        hashed_password: &str,
    ) -> anyhow::Result<UserEntity>;
}

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

/// Implements a row/type mapping for sqlx to map our user entity directly into a scanned struct from a query.
impl<'a> FromRow<'a, PgRow> for UserEntity {
    fn from_row(row: &'a PgRow) -> Result<Self, sqlx::Error> {
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

impl UserEntity {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            email: self.email,
            username: self.username,
            bio: self.bio,
            image: self.image,
            token,
        }
    }
}
