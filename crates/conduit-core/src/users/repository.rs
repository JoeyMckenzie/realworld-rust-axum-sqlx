use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

use conduit_domain::profiles::ProfileDto;
use conduit_domain::users::UserDto;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersRepository {
    async fn search_user_by_email_or_username(&self, email: &str, username: &str)
        -> anyhow::Result<Option<UserEntity>>;

    async fn create_user(&self, email: &str, username: &str, hashed_password: &str) -> anyhow::Result<UserEntity>;

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<UserEntity>;

    async fn update_user(
        &self,
        id: i64,
        email: String,
        username: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<UserEntity>;
}

#[derive(FromRow)]
pub struct UserEntity {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
}

impl UserEntity {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            id: self.id,
            email: self.email,
            username: self.username,
            bio: Some(self.bio),
            image: Some(self.image),
            token,
        }
    }

    pub fn into_profile(self, following: bool) -> ProfileDto {
        ProfileDto {
            username: self.username,
            bio: self.bio,
            image: self.image,
            following,
        }
    }
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: 1,
            bio: String::from("stub bio"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            username: String::from("stub username"),
            email: String::from("stub email"),
            password: String::from("stub password"),
            image: String::from("stub image"),
        }
    }
}
