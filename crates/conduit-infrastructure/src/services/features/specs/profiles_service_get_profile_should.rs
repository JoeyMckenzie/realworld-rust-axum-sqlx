use std::sync::Arc;

use mockall::predicate::*;

use conduit_core::errors::ConduitError;
use conduit_core::profiles::repository::{DynProfilesRepository, UserFollowEntity};
use conduit_core::profiles::service::ProfilesService;
use conduit_core::users::repository::{DynUsersRepository, UserEntity};

use crate::services::features::profiles_service::ConduitProfilesService;
use crate::services::features::specs::fixtures::ProfilesServiceTestFixture;

#[tokio::test]
async fn return_success_when_downstream_services_succeed_and_user_exists() {
    // arrange
    let mut fixture = ProfilesServiceTestFixture::default();

    fixture
        .mock_users_repository
        .expect_get_user_by_username()
        .with(eq("stub username"))
        .times(1)
        .return_once(move |_| Ok(Some(UserEntity::default())));

    fixture.mock_profiles_repository.expect_get_user_followees().times(0);

    let profiles_service = ConduitProfilesService::new(
        Arc::new(fixture.mock_users_repository) as DynUsersRepository,
        Arc::new(fixture.mock_profiles_repository) as DynProfilesRepository,
    );

    // act
    let response = profiles_service.get_profile("stub username", None).await;

    // assert
    assert!(response.is_ok());
    assert!(!response.unwrap().following);
}

#[tokio::test]
async fn call_get_user_followees_when_id_is_valid() {
    // arrange
    let mut fixture = ProfilesServiceTestFixture::default();

    let user_id = Some(2_i64);

    fixture
        .mock_users_repository
        .expect_get_user_by_username()
        .with(eq("stub username"))
        .times(1)
        .return_once(move |_| Ok(Some(UserEntity::default())));

    fixture
        .mock_profiles_repository
        .expect_get_user_followees()
        .with(eq(user_id.unwrap()))
        .times(1)
        .return_once(move |_| Ok(vec![UserFollowEntity::default()]));

    let profiles_service = ConduitProfilesService::new(
        Arc::new(fixture.mock_users_repository) as DynUsersRepository,
        Arc::new(fixture.mock_profiles_repository) as DynProfilesRepository,
    );

    // act
    let response = profiles_service.get_profile("stub username", user_id).await;

    // assert
    assert!(response.is_ok());
    assert!(response.unwrap().following);
}

#[tokio::test]
async fn return_not_found_when_user_does_not_exist() {
    // arrange
    let mut fixture = ProfilesServiceTestFixture::default();

    fixture
        .mock_users_repository
        .expect_get_user_by_username()
        .with(eq("stub username"))
        .times(1)
        .return_once(move |_| Ok(None));

    fixture.mock_profiles_repository.expect_get_user_followees().times(0);

    let profiles_service = ConduitProfilesService::new(
        Arc::new(fixture.mock_users_repository) as DynUsersRepository,
        Arc::new(fixture.mock_profiles_repository) as DynProfilesRepository,
    );

    let expected_err = ConduitError::NotFound(String::from("profile was not found")).to_string();

    // act
    let response = profiles_service.get_profile("stub username", None).await;

    // assert
    assert!(response.is_err());
    assert_eq!(response.unwrap_err().to_string(), expected_err);
}
