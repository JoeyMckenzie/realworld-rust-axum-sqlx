use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use time::Format;

use conduit_domain::articles::models::AuthorDto;
use conduit_domain::comments::CommentDto;

pub type DynCommentsRepository = Arc<dyn CommentsRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait CommentsRepository {
    async fn get_comments(&self, user_id: Option<i64>, article_id: i64) -> anyhow::Result<Vec<CommentQuery>>;
    async fn get_comment(&self, comment_id: i64) -> anyhow::Result<Option<CommentEntity>>;
    async fn create_comment(&self, article_id: i64, user_id: i64, body: String) -> anyhow::Result<CommentQuery>;
    async fn delete_comment(&self, comment_id: i64) -> anyhow::Result<()>;
}

#[derive(FromRow)]
pub struct CommentEntity {
    pub id: i64,
    pub body: String,
    pub user_id: i64,
    pub article_id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(FromRow)]
pub struct CommentQuery {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub body: String,
    pub author_username: String,
    pub author_bio: String,
    pub author_image: String,
    pub following_author: bool,
}

impl From<CommentQuery> for CommentDto {
    fn from(query: CommentQuery) -> Self {
        Self {
            id: query.id,
            created_at: query.created_at.lazy_format(Format::Rfc3339).to_string(),
            updated_at: query.updated_at.lazy_format(Format::Rfc3339).to_string(),
            body: query.body,
            author: AuthorDto {
                username: query.author_username,
                bio: query.author_bio,
                image: query.author_image,
                following: query.following_author,
            },
        }
    }
}
