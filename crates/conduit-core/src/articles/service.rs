use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use conduit_domain::articles::models::ArticleDto;

use crate::errors::ConduitResult;

pub type DynArticlesService = Arc<dyn ArticlesService + Send + Sync>;

#[automock]
#[async_trait]
pub trait ArticlesService {
    async fn create_article(
        &self,
        user_id: i64,
        title: String,
        description: String,
        body: String,
        tag_list: Vec<String>,
    ) -> ConduitResult<ArticleDto>;

    async fn update_article(
        &self,
        user_id: i64,
        slug: String,
        title: Option<String>,
        description: Option<String>,
        body: Option<String>,
    ) -> ConduitResult<ArticleDto>;

    async fn get_articles(
        &self,
        user_id: Option<i64>,
        tag: Option<String>,
        author: Option<String>,
        favorited: Option<String>,
        limit: i64,
        offset: i64,
    ) -> ConduitResult<Vec<ArticleDto>>;

    async fn get_article(&self, user_id: Option<i64>, slug: String) -> ConduitResult<ArticleDto>;

    async fn get_feed(&self, user_id: i64, limit: i64, offset: i64) -> ConduitResult<Vec<ArticleDto>>;

    async fn delete_article(&self, user_id: i64, slug: String) -> ConduitResult<()>;

    async fn favorite_article(&self, user_id: i64, slug: String) -> ConduitResult<ArticleDto>;

    async fn unfavorite_article(&self, user_id: i64, slug: String) -> ConduitResult<ArticleDto>;
}
