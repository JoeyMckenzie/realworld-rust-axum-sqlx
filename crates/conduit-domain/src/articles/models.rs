use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ArticleDto {
    #[serde(skip_serializing)]
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: i64,
    pub author: AuthorDto,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
