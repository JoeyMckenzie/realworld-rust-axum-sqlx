use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::errors::ConduitResult;

pub type DynTagsService = Arc<dyn TagsService + Send + Sync>;

#[automock]
#[async_trait]
pub trait TagsService {
    async fn get_tags(&self) -> ConduitResult<Vec<String>>;
}
