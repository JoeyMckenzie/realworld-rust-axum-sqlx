use std::collections::HashMap;

use axum::extract::Path;
use axum::{Extension, Json};
use tracing::info;

use conduit_core::errors::ConduitResult;
use conduit_core::profiles::service::DynProfilesService;
use conduit_domain::profiles::responses::ProfileResponse;

use crate::extractors::authentication_extractor::AuthenticationExtractor;

pub async fn get_profile(
    Path(params): Path<HashMap<String, String>>,
    Extension(profiles_service): Extension<DynProfilesService>,
    AuthenticationExtractor(user_id): AuthenticationExtractor,
) -> ConduitResult<Json<ProfileResponse>> {
    let username = params.get("username").unwrap();

    info!("recieved request to get profile {:?}", username);

    let profile = profiles_service.get_profile(username, user_id).await?;

    Ok(Json(ProfileResponse { profile }))
}
