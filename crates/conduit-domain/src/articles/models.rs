use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct ArticleDto {
    #[serde(skip_serializing, skip_deserializing)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthorDto {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateArticleDto {
    #[validate(required, length(min = 1))]
    pub title: Option<String>,
    #[validate(required, length(min = 1))]
    pub description: Option<String>,
    #[validate(required, length(min = 1))]
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
