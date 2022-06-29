use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use conduit_domain::profiles::ProfileDto;

use crate::errors::ConduitResult;

/// A reference counter for our profiles service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynProfilesService = Arc<dyn ProfilesService + Send + Sync>;

#[automock]
#[async_trait]
pub trait ProfilesService {
    async fn get_profile(&self, username: &str, current_user_id: Option<i64>) -> ConduitResult<ProfileDto>;

    async fn add_user_follow(&self, username: &str, current_user_id: i64) -> ConduitResult<ProfileDto>;

    async fn remove_user_follow(&self, username: &str, current_user_id: i64) -> ConduitResult<ProfileDto>;
}
