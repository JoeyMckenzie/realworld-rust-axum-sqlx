use async_trait::async_trait;
use tracing::{error, info};

use conduit_core::errors::{ConduitError, ConduitResult};
use conduit_core::users::repository::DynUsersRepository;
use conduit_core::users::service::UsersService;
use conduit_core::utils::security_service::DynSecurityService;
use conduit_core::utils::token_service::DynTokenService;
use conduit_domain::users::requests::{LoginUserDto, RegisterUserDto, UpdateUserDto};
use conduit_domain::users::UserDto;

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
            return Err(ConduitError::ObjectConflict(String::from("username or email is taken")));
        }

        info!("creating password hash for user {:?}", email);
        let hashed_password = self.security_service.hash_password(&password)?;

        info!("password hashed successfully, creating user {:?}", email);
        let created_user = self.repository.create_user(&email, &username, &hashed_password).await?;

        info!("user successfully created, generating token");
        let token = self.token_service.new_token(created_user.id, &created_user.email)?;

        Ok(created_user.into_dto(token))
    }

    async fn login_user(&self, request: LoginUserDto) -> ConduitResult<UserDto> {
        let email = request.email.unwrap();
        let attempted_password = request.password.unwrap();

        info!("searching for existing user {:?}", email);
        let existing_user = self.repository.get_user_by_email(&email).await?;

        if existing_user.is_none() {
            return Err(ConduitError::NotFound(String::from("user email does not exist")));
        }

        let user = existing_user.unwrap();

        info!("user found, verifying password hash for user {:?}", email);
        let is_valid_login_attempt = self
            .security_service
            .verify_password(&user.password, attempted_password)?;

        if !is_valid_login_attempt {
            error!("invalid login attempt for user {:?}", email);
            return Err(ConduitError::InvalidLoginAttmpt);
        }

        info!("user login successful, generating token");
        let token = self.token_service.new_token(user.id, &user.email)?;

        Ok(user.into_dto(token))
    }

    async fn get_current_user(&self, user_id: i64) -> ConduitResult<UserDto> {
        info!("retrieving user {:?}", user_id);
        let user = self.repository.get_user_by_id(user_id).await?;

        info!("user found with email {:?}, generating new token", user.email);
        let token = self.token_service.new_token(user.id, user.email.as_str())?;

        Ok(user.into_dto(token))
    }

    async fn updated_user(&self, user_id: i64, request: UpdateUserDto) -> ConduitResult<UserDto> {
        info!("retrieving user {:?}", user_id);
        let user = self.repository.get_user_by_id(user_id).await?;

        let updated_email = request.email.unwrap_or(user.email);
        let updated_username = request.username.unwrap_or(user.username);
        let updated_bio = request.bio.unwrap_or(user.bio);
        let updated_image = request.image.unwrap_or(user.image);
        let mut updated_hashed_password = user.password;

        // if the password is included on the request, hash it and update the stored password
        if request.password.is_some() {
            updated_hashed_password = self
                .security_service
                .hash_password(request.password.unwrap().as_str())?;
        }

        info!("updating user {:?}", user_id);
        let updated_user = self
            .repository
            .update_user(
                user_id,
                updated_email.clone(),
                updated_username,
                updated_hashed_password,
                updated_bio,
                updated_image,
            )
            .await?;

        info!("user {:?} updated, generating a new token", user_id);
        let token = self.token_service.new_token(user_id, updated_email.as_str())?;

        Ok(updated_user.into_dto(token))
    }
}
