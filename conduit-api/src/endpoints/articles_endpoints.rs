use axum::extract::{Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Json, Router};
use tracing::info;

use conduit_core::articles::service::DynArticlesService;
use conduit_core::errors::ConduitResult;
use conduit_domain::articles::requests::{
    CreateArticleRequest, GetArticlesApiRequest, UpdateArticleRequest, LIMIT, OFFSET,
};
use conduit_domain::articles::responses::{ArticleResponse, ArticlesResponse};
use conduit_infrastructure::service_register::ServiceRegister;

use crate::extractors::optional_authentication_extractor::OptionalAuthenticationExtractor;
use crate::extractors::required_authentication_extractor::RequiredAuthenticationExtractor;

pub struct ArticlesRouter;

impl ArticlesRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/articles", get(get_articles))
            .route("/articles", post(create_article))
            .route("/articles/:slug", get(get_article))
            .route("/articles/:slug", put(update_article))
            .route("/articles/:slug", delete(delete_article))
            .layer(Extension(service_register.articles_service))
            .layer(Extension(service_register.token_service))
    }
}

pub async fn get_articles(
    query_params: Query<GetArticlesApiRequest>,
    Extension(articles_service): Extension<DynArticlesService>,
    OptionalAuthenticationExtractor(user_id): OptionalAuthenticationExtractor,
) -> ConduitResult<Json<ArticlesResponse>> {
    info!("recieved request to retrieve articles {:?}", query_params.0);

    let articles = articles_service
        .get_articles(
            user_id,
            query_params.0.tag,
            query_params.0.author,
            query_params.0.favorited,
            query_params.0.limit.unwrap_or_else(|| LIMIT.abs()),
            query_params.0.offset.unwrap_or_else(|| OFFSET.abs()),
        )
        .await?;

    Ok(Json(ArticlesResponse { articles }))
}

pub async fn get_article(
    Path(slug): Path<String>,
    Extension(articles_service): Extension<DynArticlesService>,
    OptionalAuthenticationExtractor(user_id): OptionalAuthenticationExtractor,
) -> ConduitResult<Json<ArticleResponse>> {
    info!("recieved request to retrieve article {:?}", slug);

    let article = articles_service.get_article(user_id, slug).await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn create_article(
    Json(request): Json<CreateArticleRequest>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthenticationExtractor(user_id): RequiredAuthenticationExtractor,
) -> ConduitResult<Json<ArticleResponse>> {
    info!("recieved request to create article {:?}", request.article);

    let article = articles_service
        .create_article(
            user_id,
            request.article.title.unwrap(),
            request.article.description.unwrap(),
            request.article.body.unwrap(),
            request.article.tag_list,
        )
        .await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn update_article(
    Path(slug): Path<String>,
    Json(request): Json<UpdateArticleRequest>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthenticationExtractor(user_id): RequiredAuthenticationExtractor,
) -> ConduitResult<Json<ArticleResponse>> {
    info!("recieved request to update article {:?}", request.article);

    let article = articles_service
        .update_article(
            user_id,
            slug,
            request.article.title,
            request.article.description,
            request.article.body,
        )
        .await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn delete_article(
    Path(slug): Path<String>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthenticationExtractor(user_id): RequiredAuthenticationExtractor,
) -> ConduitResult<()> {
    info!("recieved request to delete article {:?}", slug);

    articles_service.delete_article(user_id, slug).await?;

    Ok(())
}
