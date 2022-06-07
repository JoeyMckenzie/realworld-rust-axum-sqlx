use crate::users::repository::DynUsersRepository;
use async_trait::async_trait;
use conduit_domain::users::models::UserDto;
use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto};
use std::sync::Arc;

pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[async_trait]
pub trait UsersService {
    async fn register_user(&self, request: RegisterUserDto) -> anyhow::Result<UserDto>;
    async fn login_user(&self, request: LoginUserDto) -> anyhow::Result<UserDto>;
}

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
    async fn register_user(&self, request: RegisterUserDto) -> anyhow::Result<UserDto> {
        let existing_user = self
            .repository
            .get_user_by_email_or_username(request.email.unwrap(), request.username.unwrap())
            .await;

        Ok(UserDto {
            email: String::from("email"),
            username: String::from("username"),
            bio: String::from("bio"),
            image: String::from("image"),
            token: String::from("token"),
        })
    }

    async fn login_user(&self, _request: LoginUserDto) -> anyhow::Result<UserDto> {
        todo!()
    }
}
