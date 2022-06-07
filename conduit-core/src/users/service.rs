use crate::errors::{ConduitError, ConduitResult};
use crate::users::{DynUsersRepository, UsersService};
use async_trait::async_trait;
use conduit_domain::users::models::UserDto;
use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto};
use tracing::error;

#[derive(Clone)]
pub struct UsersServiceImpl {
    repository: DynUsersRepository,
}

impl UsersServiceImpl {
    pub fn new(repository: DynUsersRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UsersService for UsersServiceImpl {
    async fn register_user(&self, request: RegisterUserDto) -> ConduitResult<UserDto> {
        let email = request.email.unwrap();
        let username = request.username.unwrap();

        let existing_user = self
            .repository
            .get_user_by_email_or_username(&email, &username)
            .await?;

        if existing_user.is_some() {
            error!("user {:?}/{:?} already exists", email, username);
            return Err(ConduitError::ObjectConflict("username or email is taken"));
        }

        Ok(UserDto {
            email: String::from("email"),
            username: String::from("username"),
            bio: String::from("bio"),
            image: String::from("image"),
            token: String::from("token"),
        })
    }

    async fn login_user(&self, _request: LoginUserDto) -> ConduitResult<UserDto> {
        todo!()
    }
}
