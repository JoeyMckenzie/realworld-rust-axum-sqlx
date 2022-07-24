use serde::{Deserialize, Serialize};

use crate::articles::models::ArticleDto;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ArticlesResponse {
    pub articles: Vec<ArticleDto>,
    #[serde(rename = "articlesCount")]
    pub articles_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ArticleResponse {
    pub article: ArticleDto,
}
