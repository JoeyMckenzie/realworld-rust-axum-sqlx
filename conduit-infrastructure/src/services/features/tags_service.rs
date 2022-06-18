use async_trait::async_trait;
use itertools::Itertools;
use tracing::info;

use conduit_core::errors::ConduitResult;
use conduit_core::tags::repository::DynTagsRepository;
use conduit_core::tags::service::TagsService;

pub struct ConduitTagsService {
    tags_repository: DynTagsRepository,
}

impl ConduitTagsService {
    pub fn new(tags_repository: DynTagsRepository) -> Self {
        Self { tags_repository }
    }
}

#[async_trait]
impl TagsService for ConduitTagsService {
    async fn get_tags(&self) -> ConduitResult<Vec<String>> {
        let tags = self
            .tags_repository
            .get_tags(vec![])
            .await?
            .into_iter()
            .map_into::<String>()
            .collect_vec();

        info!("found {:?} tags", tags.len());

        Ok(tags)
    }
}
