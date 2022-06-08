use crate::errors::{ConduitError, ConduitResult};
use crate::users::{DynUsersRepository, UsersService};
use argon2::Config;
use async_trait::async_trait;
use conduit_domain::users::models::UserDto;
use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto};
use tracing::{error, info};

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
    async fn register_user(
        &self,
        request: RegisterUserDto,
        salt: String,
    ) -> ConduitResult<UserDto> {
        let email = request.email.unwrap();
        let username = request.username.unwrap();
        let password = request.password.unwrap();
        let password_bytes = password.as_bytes();

        let existing_user = self
            .repository
            .get_user_by_email_or_username(&email, &username)
            .await?;

        if existing_user.is_some() {
            error!("user {:?}/{:?} already exists", email, username);
            return Err(ConduitError::ObjectConflict("username or email is taken"));
        }

        info!("creating password hash for user {:?}", email);
        let argon_config = Config::default();
        let hashed_password =
            argon2::hash_encoded(password_bytes, salt.as_bytes(), &argon_config).unwrap();
        let hashes_match = argon2::verify_encoded(&hashed_password, password_bytes).unwrap();

        if !hashes_match {
            error!("password hashes do not match, please verify the configuration");
            return Err(ConduitError::InternalServerError);
        }

        let created_user = self
            .repository
            .create_user(&email, &username, &hashed_password)
            .await?;

        Ok(UserDto {
            email: created_user.email,
            username: created_user.username,
            bio: created_user.bio,
            image: String::from(""),
            token: String::from(""),
        })
    }

    async fn login_user(&self, _request: LoginUserDto) -> ConduitResult<UserDto> {
        todo!()
    }
}
