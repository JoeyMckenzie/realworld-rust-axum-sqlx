use lazy_static::lazy_static;
use serde::Deserialize;
use validator::Validate;

use crate::articles::models::{CreateArticleDto, UpdateArticleDto};

lazy_static! {
    pub static ref LIMIT: i64 = 20;
    pub static ref OFFSET: i64 = 0;
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateArticleRequest {
    #[validate]
    pub article: CreateArticleDto,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub article: UpdateArticleDto,
}

#[derive(Debug, Deserialize)]
pub struct GetArticlesApiRequest {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug)]
pub struct GetArticlesServiceRequest {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

impl From<GetArticlesApiRequest> for GetArticlesServiceRequest {
    fn from(request: GetArticlesApiRequest) -> Self {
        Self {
            tag: request.tag,
            author: request.author,
            favorited: request.favorited,
            limit: request.limit.unwrap_or_else(|| LIMIT.abs()),
            offset: request.offset.unwrap_or_else(|| OFFSET.abs()),
        }
    }
}
