use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::FromRow;

use conduit_domain::articles::models::{ArticleDto, AuthorDto};

pub type DynArticlesRepository = Arc<dyn ArticlesRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait ArticlesRepository {
    async fn create_article(
        &self,
        user_id: i64,
        title: String,
        slug: String,
        description: String,
        body: String,
    ) -> anyhow::Result<UpsertArticleQuery>;

    async fn update_article(
        &self,
        id: i64,
        title: String,
        slug: String,
        description: String,
        body: String,
    ) -> anyhow::Result<UpsertArticleQuery>;

    async fn get_articles(
        &self,
        user_id: Option<i64>,
        tag: Option<String>,
        author: Option<String>,
        favorited: Option<String>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<GetArticleQuery>>;

    async fn get_article_by_slug(
        &self,
        user_id: Option<i64>,
        slug: String,
    ) -> anyhow::Result<Option<GetArticleQuery>>;

    async fn delete_article(&self, id: i64) -> anyhow::Result<()>;
}

#[derive(FromRow)]
pub struct UpsertArticleQuery {
    pub id: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub title: String,
    pub body: String,
    pub description: String,
    pub slug: String,
    pub author_username: String,
    pub author_image: String,
    pub author_bio: String,
}

#[derive(FromRow)]
pub struct GetArticleQuery {
    pub id: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub title: String,
    pub body: String,
    pub description: String,
    pub slug: String,
    pub user_id: i64,
    pub favorites: i64,
    pub favorited: bool,
    pub following_author: bool,
    pub author_username: String,
    pub author_image: String,
    pub author_bio: String,
}

impl UpsertArticleQuery {
    pub fn into_dto(self, tag_list: Vec<String>) -> ArticleDto {
        ArticleDto {
            id: self.id,
            title: self.title,
            body: self.body,
            tag_list,
            created_at: self.created_at.assume_utc().to_string(),
            updated_at: self.updated_at.assume_utc().to_string(),
            description: self.description,
            slug: self.slug,
            favorited: false,
            favorites_count: 0,
            author: AuthorDto {
                username: self.author_username,
                bio: self.author_bio,
                image: self.author_image,
                following: false,
            },
        }
    }
}

impl GetArticleQuery {
    pub fn into_dto(self, tag_list: Vec<String>) -> ArticleDto {
        ArticleDto {
            id: self.id,
            title: self.title,
            body: self.body,
            tag_list,
            created_at: self.created_at.assume_utc().to_string(),
            updated_at: self.updated_at.assume_utc().to_string(),
            description: self.description,
            slug: self.slug,
            favorited: self.favorited,
            favorites_count: self.favorites,
            author: AuthorDto {
                username: self.author_username,
                bio: self.author_bio,
                image: self.author_image,
                following: self.following_author,
            },
        }
    }
}
