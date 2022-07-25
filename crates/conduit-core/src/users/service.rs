use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto, UpdateUserDto};
use conduit_domain::users::UserDto;

use crate::errors::ConduitResult;

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersService {
    async fn register_user(&self, request: RegisterUserDto) -> ConduitResult<UserDto>;

    async fn login_user(&self, request: LoginUserDto) -> ConduitResult<UserDto>;

    async fn get_current_user(&self, user_id: i64) -> ConduitResult<UserDto>;

    async fn updated_user(&self, user_id: i64, request: UpdateUserDto) -> ConduitResult<UserDto>;
}
