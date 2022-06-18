use axum::routing::get;
use axum::{Extension, Json, Router};
use tracing::info;

use conduit_core::errors::ConduitResult;
use conduit_core::tags::service::DynTagsService;
use conduit_domain::tags::responses::TagsResponse;
use conduit_infrastructure::service_register::ServiceRegister;

pub struct TagsRouter;

impl TagsRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/tags", get(get_tags))
            .layer(Extension(service_register.tags_service))
    }
}

pub async fn get_tags(
    Extension(tags_service): Extension<DynTagsService>,
) -> ConduitResult<Json<TagsResponse>> {
    info!("recieved request to retrieve all tags");

    let tags = tags_service.get_tags().await?;

    Ok(Json(TagsResponse { tags }))
}
