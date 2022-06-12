use std::collections::HashMap;

use axum::extract::Path;
use axum::{Extension, Json};
use tracing::info;

use conduit_core::errors::ConduitResult;
use conduit_core::profiles::service::DynProfilesService;
use conduit_domain::profiles::responses::ProfileResponse;

use crate::extractors::optional_authentication_extractor::OptionalAuthenticationExtractor;
use crate::extractors::required_authentication_extractor::RequiredAuthenticationExtractor;

pub async fn get_profile(
    Path(params): Path<HashMap<String, String>>,
    Extension(profiles_service): Extension<DynProfilesService>,
    OptionalAuthenticationExtractor(user_id): OptionalAuthenticationExtractor,
) -> ConduitResult<Json<ProfileResponse>> {
    let username = params.get("username").unwrap();

    info!("recieved request to get profile {:?}", username);

    let profile = profiles_service.get_profile(username, user_id).await?;

    Ok(Json(ProfileResponse { profile }))
}

pub async fn follow_user(
    Path(params): Path<HashMap<String, String>>,
    Extension(profiles_service): Extension<DynProfilesService>,
    RequiredAuthenticationExtractor(user_id): RequiredAuthenticationExtractor,
) -> ConduitResult<Json<ProfileResponse>> {
    let username = params.get("username").unwrap();

    info!(
        "recieved request to follow profile {:?} from user ID {:?}",
        username, user_id
    );

    let profile = profiles_service.add_user_follow(username, user_id).await?;

    Ok(Json(ProfileResponse { profile }))
}

pub async fn unfollow_user(
    Path(params): Path<HashMap<String, String>>,
    Extension(profiles_service): Extension<DynProfilesService>,
    RequiredAuthenticationExtractor(user_id): RequiredAuthenticationExtractor,
) -> ConduitResult<Json<ProfileResponse>> {
    let username = params.get("username").unwrap();

    info!(
        "recieved request to unfollow profile {:?} from user ID {:?}",
        username, user_id
    );

    let profile = profiles_service
        .remove_user_follow(username, user_id)
        .await?;

    Ok(Json(ProfileResponse { profile }))
}
