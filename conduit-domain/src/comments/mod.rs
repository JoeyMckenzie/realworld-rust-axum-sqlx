use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::articles::models::AuthorDto;

pub mod requests;
pub mod responses;

#[derive(Debug, Serialize)]
pub struct CommentDto {
    pub id: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub author: AuthorDto,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentDto {
    #[validate(required)]
    pub body: Option<String>,
}
