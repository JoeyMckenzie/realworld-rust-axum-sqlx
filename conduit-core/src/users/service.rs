use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use conduit_domain::users::models::UserDto;
use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto};

use crate::errors::ConduitResult;
use crate::users::repository::UserEntity;

/// A reference counter for our user service allows us safely pass instances user services
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersService {
    async fn register_user(&self, request: RegisterUserDto) -> ConduitResult<UserDto>;
    async fn login_user(&self, request: LoginUserDto) -> ConduitResult<UserDto>;
    async fn get_current_user(&self, token: String) -> ConduitResult<UserDto>;
}

/// Implements a mapping from our `UserEntity` into the view model `UserDto` to be consumed by API clients.
impl From<UserEntity> for UserDto {
    fn from(entity: UserEntity) -> Self {
        UserDto {
            email: entity.email,
            username: entity.username,
            bio: entity.bio,
            image: entity.image,
            token: String::from(""),
        }
    }
}
