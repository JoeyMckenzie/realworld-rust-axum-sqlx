use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::PrimitiveDateTime;

pub type DynTagsRepository = Arc<dyn TagsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait TagsRepository {
    async fn get_tags(&self, tags: Vec<String>) -> anyhow::Result<Vec<TagEntity>>;

    async fn create_tags(&self, tags: Vec<String>) -> anyhow::Result<Vec<TagEntity>>;

    async fn get_article_tags_by_article_id(
        &self,
        article_id: i64,
    ) -> anyhow::Result<Vec<ArticleTagQuery>>;

    async fn get_article_tags_article_ids(
        &self,
        article_ids: Vec<i64>,
    ) -> anyhow::Result<Vec<ArticleTagQuery>>;

    async fn create_article_tags(&self, tags: Vec<(i64, i64)>) -> anyhow::Result<()>;
}

pub struct TagEntity {
    pub id: i64,
    pub tag: String,
    pub created_at: PrimitiveDateTime,
}

pub struct ArticleTagEntity {
    pub id: i64,
    pub tag_id: i64,
    pub article_id: i64,
    pub created_at: PrimitiveDateTime,
}

pub struct ArticleTagQuery {
    pub id: i64,
    pub tag_id: i64,
    pub article_id: i64,
    pub tag: String,
}

impl From<TagEntity> for String {
    fn from(entity: TagEntity) -> Self {
        entity.tag
    }
}
