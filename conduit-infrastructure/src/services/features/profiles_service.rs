use async_trait::async_trait;
use tracing::info;

use conduit_core::errors::{ConduitError, ConduitResult};
use conduit_core::profiles::repository::DynProfilesRepository;
use conduit_core::profiles::service::ProfilesService;
use conduit_core::users::repository::DynUsersRepository;
use conduit_domain::profiles::ProfileDto;

#[derive(Clone)]
pub struct ConduitProfilesService {
    users_repository: DynUsersRepository,
    profiles_repository: DynProfilesRepository,
}

impl ConduitProfilesService {
    pub fn new(
        users_repository: DynUsersRepository,
        profiles_repository: DynProfilesRepository,
    ) -> Self {
        Self {
            users_repository,
            profiles_repository,
        }
    }
}

#[async_trait]
impl ProfilesService for ConduitProfilesService {
    async fn get_profile(
        &self,
        username: &String,
        current_user_id: i64,
    ) -> ConduitResult<ProfileDto> {
        info!("retrieving profile for user {:?}", username);
        let user = self
            .users_repository
            .get_user_by_username(username.as_str())
            .await?;

        if user.is_none() {
            return Err(ConduitError::NotFound(String::from(
                "profile was not found",
            )));
        }

        info!("retrieving followee list for user {:?}", username);

        Ok(user.unwrap().into_profile(false))
    }
}
