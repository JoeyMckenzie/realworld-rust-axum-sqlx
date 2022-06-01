use crate::users::repository::UsersRepository;
use async_trait::async_trait;
use conduit_domain::users::models::{User, UserDto};
use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto};
use std::sync::Arc;

pub type DynUserService = Arc<dyn UsersService + Send + Sync>;

#[async_trait]
pub trait UsersService {
    async fn register_user(&self, request: RegisterUserDto) -> anyhow::Result<UserDto>;
    async fn login_user(&self, request: LoginUserDto) -> anyhow::Result<UserDto>;
}
