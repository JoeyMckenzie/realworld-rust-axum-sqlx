use std::sync::Arc;

use mockall::predicate::*;

use conduit_core::services::security_service::{DynSecurityService, MockSecurityService};
use conduit_core::services::token_service::{DynTokenService, MockTokenService};
use conduit_core::users::repository::{DynUsersRepository, MockUsersRepository, UserEntity};
use conduit_core::users::service::UsersService;
use conduit_domain::users::requests::RegisterUserDto;

use crate::services::features::users_service::ConduitUsersService;

#[tokio::test]
async fn register_user_returns_success_when_downstream_services_succeed() {
    // arrange
    let mut mock_repository = MockUsersRepository::new();
    let mut mock_token_service = MockTokenService::new();
    let mut mock_security_service = MockSecurityService::new();

    let request = RegisterUserDto {
        username: Some(String::from("stub username")),
        email: Some(String::from("stub email")),
        password: Some(String::from("stub password")),
    };

    mock_repository
        .expect_search_user_by_email_or_username()
        .with(eq("stub email"), eq("stub username"))
        .times(1)
        .return_once(move |_, _| Ok(None));

    mock_repository
        .expect_create_user()
        .with(eq("stub email"), eq("stub username"), eq("hashed password"))
        .times(1)
        .return_once(move |_, _, _| Ok(UserEntity::default()));

    mock_security_service
        .expect_hash_password()
        .with(eq("stub password"))
        .times(1)
        .return_once(move |_| Ok(String::from("hashed password")));

    mock_token_service
        .expect_new_token()
        .with(eq(1), eq("stub email"))
        .times(1)
        .return_once(move |_, _| Ok(String::from("stub token")));

    let users_service = ConduitUsersService::new(
        Arc::new(mock_repository) as DynUsersRepository,
        Arc::new(mock_security_service) as DynSecurityService,
        Arc::new(mock_token_service) as DynTokenService,
    );

    // act
    let response = users_service.register_user(request).await;

    // assert
    assert!(response.is_ok());
}

#[tokio::test]
async fn register_user_returns_error_when_user_exixsts() {
    // arrange
    let mut mock_repository = MockUsersRepository::new();
    let mut mock_token_service = MockTokenService::new();
    let mut mock_security_service = MockSecurityService::new();

    let request = RegisterUserDto {
        username: Some(String::from("stub username")),
        email: Some(String::from("stub email")),
        password: Some(String::from("stub password")),
    };

    mock_repository
        .expect_search_user_by_email_or_username()
        .with(eq("stub email"), eq("stub username"))
        .times(1)
        .return_once(move |_, _| Ok(Some(UserEntity::default())));

    mock_repository
        .expect_create_user()
        .with(eq("stub email"), eq("stub username"), eq("hashed password"))
        .times(0);

    mock_security_service
        .expect_hash_password()
        .with(eq("stub password"))
        .times(0);

    mock_token_service
        .expect_new_token()
        .with(eq(1), eq("stub email"))
        .times(0);

    let users_service = ConduitUsersService::new(
        Arc::new(mock_repository) as DynUsersRepository,
        Arc::new(mock_security_service) as DynSecurityService,
        Arc::new(mock_token_service) as DynTokenService,
    );

    // act
    let response = users_service.register_user(request).await;

    // assert
    assert!(response.is_err());
}
