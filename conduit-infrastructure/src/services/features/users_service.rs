use async_trait::async_trait;
use tracing::{error, info};

use conduit_core::errors::{ConduitError, ConduitResult};
use conduit_core::services::security_service::DynSecurityService;
use conduit_core::services::token_service::DynTokenService;
use conduit_core::users::repository::DynUsersRepository;
use conduit_core::users::service::UsersService;
use conduit_domain::users::models::UserDto;
use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto};

#[derive(Clone)]
pub struct ConduitUsersService {
    repository: DynUsersRepository,
    security_service: DynSecurityService,
    token_service: DynTokenService,
}

impl ConduitUsersService {
    pub fn new(
        repository: DynUsersRepository,
        security_service: DynSecurityService,
        token_service: DynTokenService,
    ) -> Self {
        Self {
            repository,
            security_service,
            token_service,
        }
    }
}

#[async_trait]
impl UsersService for ConduitUsersService {
    async fn register_user(&self, request: RegisterUserDto) -> ConduitResult<UserDto> {
        let email = request.email.unwrap();
        let username = request.username.unwrap();
        let password = request.password.unwrap();

        let existing_user = self
            .repository
            .search_user_by_email_or_username(&email, &username)
            .await?;

        if existing_user.is_some() {
            error!("user {:?}/{:?} already exists", email, username);
            return Err(ConduitError::ObjectConflict(String::from(
                "username or email is taken",
            )));
        }

        info!("creating password hash for user {:?}", email);
        let hashed_password = self.security_service.hash_password(&password)?;

        info!("password hashed successfully, creating user {:?}", email);
        let created_user = self
            .repository
            .create_user(&email, &username, &hashed_password)
            .await?;

        info!("user successfully created, generating token");
        let token = self
            .token_service
            .new_token(created_user.id, &created_user.email)?;

        Ok(created_user.into_dto(token))
    }

    async fn login_user(&self, request: LoginUserDto) -> ConduitResult<UserDto> {
        let email = request.email.unwrap();
        let attempted_password = request.password.unwrap();

        info!("searching for existing user {:?}", email);
        let existing_user = self.repository.get_user_by_email(&email).await?;

        info!("user found, verifying password hash for user {:?}", email);
        let is_valid_login_attempt = self
            .security_service
            .verify_password(&existing_user.password, attempted_password)?;

        if !is_valid_login_attempt {
            error!("invalid login attempt for user {:?}", email);
            return Err(ConduitError::InvalidLoginAttmpt);
        }

        info!("user login successful, generating token");
        let token = self
            .token_service
            .new_token(existing_user.id, &existing_user.email)?;

        Ok(existing_user.into_dto(token))
    }

    async fn get_current_user(&self, token: String) -> ConduitResult<UserDto> {
        todo!()
    }
}
