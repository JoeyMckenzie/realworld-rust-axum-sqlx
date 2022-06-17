use serde::Serialize;

use crate::articles::models::ArticleDto;

#[derive(Debug, Serialize)]
pub struct ArticlesResponse {
    pub articles: Vec<ArticleDto>,
}

#[derive(Debug, Serialize)]
pub struct ArticleResponse {
    pub article: ArticleDto,
}
