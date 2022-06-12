use std::sync::Arc;

use mockall::predicate::*;

use conduit_core::profiles::repository::{
    DynProfilesRepository, MockProfilesRepository, UserFollowEntity,
};
use conduit_core::profiles::service::ProfilesService;
use conduit_core::users::repository::{DynUsersRepository, MockUsersRepository, UserEntity};

use crate::services::features::profiles_service::ConduitProfilesService;

struct ProfilesServiceTestFixture {
    mock_profiles_repository: MockProfilesRepository,
    mock_users_repository: MockUsersRepository,
}

impl ProfilesServiceTestFixture {
    fn new() -> Self {
        Self {
            mock_profiles_repository: MockProfilesRepository::new(),
            mock_users_repository: MockUsersRepository::new(),
        }
    }
}

#[tokio::test]
async fn get_profile_returns_success_when_downstream_services_succeed_and_user_exists() {
    // arrange
    let mut fixture = ProfilesServiceTestFixture::new();

    fixture
        .mock_users_repository
        .expect_get_user_by_username()
        .with(eq("stub username"))
        .times(1)
        .return_once(move |_| Ok(Some(UserEntity::default())));

    fixture
        .mock_profiles_repository
        .expect_get_user_followees()
        .times(0);

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
async fn get_profile_calls_get_user_followees_when_id_is_valid() {
    // arrange
    let mut fixture = ProfilesServiceTestFixture::new();

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
