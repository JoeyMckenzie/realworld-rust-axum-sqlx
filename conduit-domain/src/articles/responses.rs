use serde::Serialize;

use crate::articles::models::ArticleDto;

#[derive(Debug, Serialize)]
pub struct ArticlesResponse {
    pub articles: Vec<ArticleDto>,
    #[serde(rename = "articlesCount")]
    pub articles_count: usize,
}

#[derive(Debug, Serialize)]
pub struct ArticleResponse {
    pub article: ArticleDto,
}
