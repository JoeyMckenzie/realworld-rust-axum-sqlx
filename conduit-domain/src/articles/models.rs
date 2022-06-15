use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleDto {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub favorited: bool,
    pub favorites_count: i64,
    pub author: AuthorDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorDto {
    pub username: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateArticleDto {
    #[validate(required)]
    pub title: Option<String>,
    #[validate(required)]
    pub description: Option<String>,
    #[validate(required)]
    pub body: Option<String>,
    #[serde(rename = "camelCase")]
    pub tag_list: Vec<String>,
}
