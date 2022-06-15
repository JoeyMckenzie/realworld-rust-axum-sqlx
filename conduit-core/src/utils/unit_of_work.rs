use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::articles::repository::DynArticlesRepository;
use crate::errors::ConduitResult;
use crate::profiles::repository::DynProfilesRepository;
use crate::tags::repository::DynTagsRepository;
use crate::users::repository::DynUsersRepository;

/// A security service for handling JWT authentication.
pub type DynUnitOfWork = Arc<dyn UnitOfWork + Send + Sync>;

#[automock]
#[async_trait]
pub trait UnitOfWork {
    fn users_repository(&self) -> DynUsersRepository;
    fn profiles_repository(&self) -> DynProfilesRepository;
    fn articles_repository(&self) -> DynArticlesRepository;
    fn tags_repository(&self) -> DynTagsRepository;
    async fn begin(&self) -> ConduitResult<()>;
    async fn commit(&self) -> ConduitResult<()>;
    async fn rollback(&self) -> ConduitResult<()>;
}
