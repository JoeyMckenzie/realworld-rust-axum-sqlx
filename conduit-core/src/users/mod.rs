use async_trait::async_trait;
use conduit_domain::users::models::UserDto;
use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto};
use sqlx::postgres::PgRow;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::{FromRow, Row};
use std::sync::Arc;

pub mod repository;
pub mod service;

/// A reference counter for our user service allows us safely pass instances user services
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[async_trait]
pub trait UsersService {
    async fn register_user(&self, request: RegisterUserDto) -> anyhow::Result<UserDto>;
    async fn login_user(&self, request: LoginUserDto) -> anyhow::Result<UserDto>;
}

#[async_trait]
pub trait UsersRepository {
    async fn get_user_by_email_or_username(
        &self,
        email: String,
        username: String,
    ) -> anyhow::Result<Option<UserEntity>>;
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
