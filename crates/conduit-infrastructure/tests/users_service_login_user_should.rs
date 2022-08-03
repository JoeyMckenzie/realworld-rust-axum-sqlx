use std::sync::Arc;

use conduit_core::users::service::UsersService;
use conduit_infrastructure::mocks::UsersServiceTestFixture;
use mockall::predicate::*;

use conduit_core::users::repository::{DynUsersRepository, UserEntity};
use conduit_core::utils::security_service::DynSecurityService;
use conduit_core::utils::token_service::DynTokenService;
use conduit_domain::users::requests::LoginUserDto;

use conduit_infrastructure::services::users_service::ConduitUsersService;

#[tokio::test]
async fn return_success_when_downstream_services_succeedand_user_exists() {
    // arrange
    let mut fixture = UsersServiceTestFixture::default();

    fixture
        .mock_repository
        .expect_get_user_by_email()
        .with(eq("stub email"))
        .times(1)
        .return_once(move |_| Ok(Some(UserEntity::default())));

    fixture
        .mock_security_service
        .expect_verify_password()
        .with(eq("hashed password"), eq("stub password".to_string()))
        .times(1)
        .return_once(move |_, _| Ok(true));

    fixture
        .mock_token_service
        .expect_new_token()
        .with(eq(1_i64), eq("stub email"))
        .times(1)
        .return_once(move |_, _| Ok(String::from("stub token")));

    let users_service = ConduitUsersService::new(
        Arc::new(fixture.mock_repository) as DynUsersRepository,
        Arc::new(fixture.mock_security_service) as DynSecurityService,
        Arc::new(fixture.mock_token_service) as DynTokenService,
    );

    // act
    let response = users_service.login_user(LoginUserDto::new_stub()).await;

    // assert
    assert!(response.is_ok());
}

#[tokio::test]
async fn return_error_when_user_password_is_invalid() {
    // arrange
    let mut fixture = UsersServiceTestFixture::default();

    fixture
        .mock_repository
        .expect_get_user_by_email()
        .with(eq("stub email"))
        .times(1)
        .return_once(move |_| Ok(Some(UserEntity::default())));

    fixture
        .mock_security_service
        .expect_verify_password()
        .with(eq("hashed password"), eq("stub password".to_string()))
        .times(1)
        .return_once(move |_, _| Ok(false));

    fixture.mock_token_service.expect_new_token().times(0);

    let users_service = ConduitUsersService::new(
        Arc::new(fixture.mock_repository) as DynUsersRepository,
        Arc::new(fixture.mock_security_service) as DynSecurityService,
        Arc::new(fixture.mock_token_service) as DynTokenService,
    );

    // act
    let response = users_service.login_user(LoginUserDto::new_stub()).await;

    // assert
    assert!(response.is_err());
}

#[tokio::test]
async fn return_error_when_user_exixst_does_not_exist() {
    // arrange
    let mut fixture = UsersServiceTestFixture::default();

    fixture
        .mock_repository
        .expect_get_user_by_email()
        .with(eq("stub email"))
        .times(1)
        .return_once(move |_| Ok(None));

    fixture.mock_security_service.expect_verify_password().times(0);

    fixture.mock_token_service.expect_new_token().times(0);

    let users_service = ConduitUsersService::new(
        Arc::new(fixture.mock_repository) as DynUsersRepository,
        Arc::new(fixture.mock_security_service) as DynSecurityService,
        Arc::new(fixture.mock_token_service) as DynTokenService,
    );

    // act
    let response = users_service.login_user(LoginUserDto::new_stub()).await;

    // assert
    assert!(response.is_err());
}
