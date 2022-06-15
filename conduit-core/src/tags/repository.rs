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

    async fn get_article_tags(&self, article_id: i64) -> anyhow::Result<Vec<ArticleTagEntity>>;

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
