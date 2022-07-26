use conduit_domain::articles::{models::CreateArticleDto, requests::CreateArticleRequest, responses::ArticleResponse};
use lazy_static::lazy_static;

use crate::utilities::{
    errors::{ConduitWebError, ConduitWebResult},
    http::post,
};

lazy_static! {
    static ref ARTICLES_ENDPOINT: &'static str = "/api/articles";
}

pub async fn create_article(
    title: String,
    description: String,
    body: String,
    tags: Vec<String>,
) -> ConduitWebResult<ArticleResponse> {
    let article_dto = CreateArticleDto {
        title: Some(title.clone()),
        description: Some(description),
        body: Some(body),
        tag_list: tags,
    };

    let create_article_response = post::<ArticleResponse, CreateArticleRequest>(
        *ARTICLES_ENDPOINT,
        CreateArticleRequest { article: article_dto },
    )
    .await;

    if let Ok(article_response) = create_article_response {
        return Ok(article_response);
    }

    Err(ConduitWebError::ArticleNotCreated)
}
