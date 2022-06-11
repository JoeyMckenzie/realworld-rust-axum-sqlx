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
        username: &str,
        current_user_id: Option<i64>,
    ) -> ConduitResult<ProfileDto> {
        info!("retrieving profile for user {:?}", username);
        let user = self.users_repository.get_user_by_username(username).await?;

        if user.is_none() {
            return Err(ConduitError::NotFound(String::from(
                "profile was not found",
            )));
        }

        // in the case a token is passed and validly extracted, pull the list of users they're following to see if the profile is included
        if let Some(user_id) = current_user_id {
            info!("retrieving followee list for user {:?}", username);
            let users_following_list = self.profiles_repository.get_user_followees(user_id).await?;

            if users_following_list.is_empty() {
                return Ok(user.unwrap().into_profile(false));
            }

            let is_following = users_following_list
                .into_iter()
                .any(|followee| followee.follower_id == user_id);

            return Ok(user.unwrap().into_profile(is_following));
        }

        Ok(user.unwrap().into_profile(false))
    }
}
