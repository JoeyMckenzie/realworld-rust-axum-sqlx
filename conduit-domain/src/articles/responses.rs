use serde::{Deserialize, Serialize};

use crate::articles::models::ArticleDto;

#[derive(Debug, Serialize)]
pub struct GetArticlesResponse {
    pub articles: Vec<ArticleDto>,
}

#[derive(Debug, Serialize)]
pub struct GetArticleResponse {
    pub article: ArticleDto,
}
