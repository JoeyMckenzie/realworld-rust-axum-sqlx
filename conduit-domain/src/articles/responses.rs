use serde::{Deserialize, Serialize};

use crate::articles::models::ArticleDto;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct GetArticlesResponse {
    pub articles: Vec<ArticleDto>,
}
