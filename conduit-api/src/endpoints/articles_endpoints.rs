use axum::extract::{Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Json, Router};
use tracing::info;

use conduit_core::articles::service::DynArticlesService;
use conduit_core::comments::service::DynCommentsService;
use conduit_core::errors::ConduitResult;
use conduit_domain::articles::requests::{
    CreateArticleRequest, GetArticlesApiRequest, UpdateArticleRequest, LIMIT, OFFSET,
};
use conduit_domain::articles::responses::{ArticleResponse, ArticlesResponse};
use conduit_domain::comments::requests::CreateCommentRequest;
use conduit_domain::comments::responses::{CommentResponse, CommentsResponse};
use conduit_infrastructure::service_register::ServiceRegister;

use crate::extractors::optional_authentication_extractor::OptionalAuthentication;
use crate::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::extractors::validation_extractor::ValidationExtractor;

pub struct ArticlesRouter;

impl ArticlesRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/articles", get(get_articles))
            .route("/articles", post(create_article))
            .route("/articles/:slug", get(get_article))
            .route("/articles/:slug", put(update_article))
            .route("/articles/:slug", delete(delete_article))
            .route("/articles/:slug/favorite", post(favorite_article))
            .route("/articles/:slug/favorite", delete(unfavorite_article))
            .route("/articles/:slug/comments", get(get_comments))
            .route("/articles/:slug/comments", post(add_comment))
            .route("/articles/:slug/comments/:id", delete(remove_comment))
            .layer(Extension(service_register.articles_service))
            .layer(Extension(service_register.comments_service))
            .layer(Extension(service_register.token_service))
    }
}

pub async fn get_articles(
    query_params: Query<GetArticlesApiRequest>,
    Extension(articles_service): Extension<DynArticlesService>,
    OptionalAuthentication(user_id): OptionalAuthentication,
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
    OptionalAuthentication(user_id): OptionalAuthentication,
) -> ConduitResult<Json<ArticleResponse>> {
    info!("recieved request to retrieve article {:?}", slug);

    let article = articles_service.get_article(user_id, slug).await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn create_article(
    ValidationExtractor(request): ValidationExtractor<CreateArticleRequest>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
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
    RequiredAuthentication(user_id): RequiredAuthentication,
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
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> ConduitResult<()> {
    info!("recieved request to delete article {:?}", slug);

    articles_service.delete_article(user_id, slug).await?;

    Ok(())
}

pub async fn favorite_article(
    Path(slug): Path<String>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> ConduitResult<Json<ArticleResponse>> {
    info!("recieved request to favorite article {:?}", slug);

    let article = articles_service.favorite_article(user_id, slug).await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn unfavorite_article(
    Path(slug): Path<String>,
    Extension(articles_service): Extension<DynArticlesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> ConduitResult<Json<ArticleResponse>> {
    info!("recieved request to unfavorite article {:?}", slug);

    let article = articles_service.unfavorite_article(user_id, slug).await?;

    Ok(Json(ArticleResponse { article }))
}

pub async fn get_comments(
    Path(slug): Path<String>,
    Extension(comments_service): Extension<DynCommentsService>,
    OptionalAuthentication(user_id): OptionalAuthentication,
) -> ConduitResult<Json<CommentsResponse>> {
    info!(
        "recieved request to retrieve comments for article {:?}",
        slug
    );

    let comments = comments_service.get_comments(user_id, slug).await?;

    Ok(Json(CommentsResponse { comments }))
}

pub async fn add_comment(
    Path(slug): Path<String>,
    ValidationExtractor(request): ValidationExtractor<CreateCommentRequest>,
    Extension(comments_service): Extension<DynCommentsService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> ConduitResult<Json<CommentResponse>> {
    info!("recieved request to add comment for article {:?}", slug);

    let comment = comments_service
        .add_comment(user_id, slug, request.comment.body.unwrap())
        .await?;

    Ok(Json(CommentResponse { comment }))
}

pub async fn remove_comment(
    Path(comment_id): Path<i64>,
    Extension(comments_service): Extension<DynCommentsService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> ConduitResult<()> {
    info!("recieved request to remove comment {:?}", comment_id);

    comments_service.remove_comment(user_id, comment_id).await?;

    Ok(())
}
