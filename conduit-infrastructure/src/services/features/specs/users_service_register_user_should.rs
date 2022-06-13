use std::sync::Arc;

use mockall::predicate::*;

use conduit_core::users::repository::{DynUsersRepository, UserEntity};
use conduit_core::users::service::UsersService;
use conduit_core::utils::security_service::DynSecurityService;
use conduit_core::utils::token_service::DynTokenService;
use conduit_domain::users::requests::RegisterUserDto;

use crate::services::features::specs::fixtures::UsersServiceTestFixture;
use crate::services::features::users_service::ConduitUsersService;

#[tokio::test]
async fn return_success_when_downstream_services_succeed() {
    // arrange
    let mut fixture = UsersServiceTestFixture::default();

    fixture
        .mock_repository
        .expect_search_user_by_email_or_username()
        .with(eq("stub email"), eq("stub username"))
        .times(1)
        .return_once(move |_, _| Ok(None));

    fixture
        .mock_repository
        .expect_create_user()
        .with(eq("stub email"), eq("stub username"), eq("hashed password"))
        .times(1)
        .return_once(move |_, _, _| Ok(UserEntity::default()));

    fixture
        .mock_security_service
        .expect_hash_password()
        .with(eq("stub password"))
        .times(1)
        .return_once(move |_| Ok(String::from("hashed password")));

    fixture
        .mock_token_service
        .expect_new_token()
        .with(eq(1), eq("stub email"))
        .times(1)
        .return_once(move |_, _| Ok(String::from("stub token")));

    let users_service = ConduitUsersService::new(
        Arc::new(fixture.mock_repository) as DynUsersRepository,
        Arc::new(fixture.mock_security_service) as DynSecurityService,
        Arc::new(fixture.mock_token_service) as DynTokenService,
    );

    // act
    let response = users_service
        .register_user(RegisterUserDto::new_stub())
        .await;

    // assert
    assert!(response.is_ok());
}

#[tokio::test]
async fn return_error_when_user_exixsts() {
    // arrange
    let mut fixture = UsersServiceTestFixture::default();

    fixture
        .mock_repository
        .expect_search_user_by_email_or_username()
        .with(eq("stub email"), eq("stub username"))
        .times(1)
        .return_once(move |_, _| Ok(Some(UserEntity::default())));

    fixture.mock_repository.expect_create_user().times(0);

    fixture
        .mock_security_service
        .expect_hash_password()
        .times(0);

    fixture.mock_token_service.expect_new_token().times(0);

    let users_service = ConduitUsersService::new(
        Arc::new(fixture.mock_repository) as DynUsersRepository,
        Arc::new(fixture.mock_security_service) as DynSecurityService,
        Arc::new(fixture.mock_token_service) as DynTokenService,
    );

    // act
    let response = users_service
        .register_user(RegisterUserDto::new_stub())
        .await;

    // assert
    assert!(response.is_err());
}
