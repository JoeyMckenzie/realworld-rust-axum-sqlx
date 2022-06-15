use axum::extract::Query;
use axum::routing::get;
use axum::{Extension, Json, Router};
use tracing::info;

use conduit_core::articles::service::DynArticlesService;
use conduit_core::errors::ConduitResult;
use conduit_domain::articles::requests::{GetArticlesApiRequest, LIMIT, OFFSET};
use conduit_domain::articles::responses::GetArticlesResponse;
use conduit_infrastructure::service_register::ServiceRegister;

use crate::extractors::optional_authentication_extractor::OptionalAuthenticationExtractor;

pub struct ArticlesRouter;

impl ArticlesRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/articles/:username", get(get_articles))
            .layer(Extension(service_register.articles_service))
    }
}

pub async fn get_articles(
    query_params: Query<GetArticlesApiRequest>,
    Extension(articles_service): Extension<DynArticlesService>,
    OptionalAuthenticationExtractor(user_id): OptionalAuthenticationExtractor,
) -> ConduitResult<Json<GetArticlesResponse>> {
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

    Ok(Json(GetArticlesResponse { articles }))
}
