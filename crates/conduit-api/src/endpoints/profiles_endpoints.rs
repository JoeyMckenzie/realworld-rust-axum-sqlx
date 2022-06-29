use std::collections::HashMap;

use axum::extract::Path;
use axum::routing::{delete, get, post};
use axum::{Extension, Json, Router};
use tracing::info;

use conduit_core::errors::ConduitResult;
use conduit_core::profiles::service::DynProfilesService;
use conduit_domain::profiles::responses::ProfileResponse;
use conduit_infrastructure::service_register::ServiceRegister;

use crate::extractors::optional_authentication_extractor::OptionalAuthentication;
use crate::extractors::required_authentication_extractor::RequiredAuthentication;

pub struct ProfilesRouter;

impl ProfilesRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/profiles/:username", get(get_profile))
            .route("/profiles/:username/follow", post(follow_user))
            .route("/profiles/:username/follow", delete(unfollow_user))
            .layer(Extension(service_register.profiles_service))
            .layer(Extension(service_register.token_service))
    }
}

pub async fn get_profile(
    Path(params): Path<HashMap<String, String>>,
    Extension(profiles_service): Extension<DynProfilesService>,
    OptionalAuthentication(user_id): OptionalAuthentication,
) -> ConduitResult<Json<ProfileResponse>> {
    let username = params.get("username").unwrap();

    info!("recieved request to get profile {:?}", username);

    let profile = profiles_service.get_profile(username, user_id).await?;

    Ok(Json(ProfileResponse { profile }))
}

pub async fn follow_user(
    Path(params): Path<HashMap<String, String>>,
    Extension(profiles_service): Extension<DynProfilesService>,
    RequiredAuthentication(user_id): RequiredAuthentication,
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
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> ConduitResult<Json<ProfileResponse>> {
    let username = params.get("username").unwrap();

    info!(
        "recieved request to unfollow profile {:?} from user ID {:?}",
        username, user_id
    );

    let profile = profiles_service.remove_user_follow(username, user_id).await?;

    Ok(Json(ProfileResponse { profile }))
}
