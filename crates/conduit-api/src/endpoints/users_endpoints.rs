use axum::routing::{get, post, put};
use axum::{Extension, Json, Router};
use tracing::info;

use conduit_core::errors::ConduitResult;
use conduit_core::users::service::DynUsersService;
use conduit_domain::users::requests::{LoginUserRequest, RegisterUserRequest, UpdateUserRequest};
use conduit_domain::users::responses::UserAuthenicationResponse;
use conduit_infrastructure::service_register::ServiceRegister;

use crate::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::extractors::validation_extractor::ValidationExtractor;

pub struct UsersRouter;

impl UsersRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/users", post(UsersRouter::register_user_endpoint))
            .route("/users/login", post(UsersRouter::login_user_endpoint))
            .route("/user", get(UsersRouter::get_current_user_endpoint))
            .route("/user", put(UsersRouter::update_user_endpoint))
            .layer(Extension(service_register.users_service))
            .layer(Extension(service_register.token_service))
    }

    pub async fn register_user_endpoint(
        ValidationExtractor(request): ValidationExtractor<RegisterUserRequest>,
        Extension(users_service): Extension<DynUsersService>,
    ) -> ConduitResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to create user {:?}/{:?}",
            request.user.email.as_ref().unwrap(),
            request.user.username.as_ref().unwrap()
        );

        let created_user = users_service.register_user(request.user).await?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
    }

    pub async fn login_user_endpoint(
        ValidationExtractor(request): ValidationExtractor<LoginUserRequest>,
        Extension(users_service): Extension<DynUsersService>,
    ) -> ConduitResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to login user {:?}",
            request.user.email.as_ref().unwrap()
        );

        let created_user = users_service.login_user(request.user).await?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
    }

    pub async fn get_current_user_endpoint(
        RequiredAuthentication(user_id): RequiredAuthentication,
        Extension(users_service): Extension<DynUsersService>,
    ) -> ConduitResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to retrieve current user");

        let current_user = users_service.get_current_user(user_id).await?;

        Ok(Json(UserAuthenicationResponse { user: current_user }))
    }

    pub async fn update_user_endpoint(
        RequiredAuthentication(user_id): RequiredAuthentication,
        Extension(users_service): Extension<DynUsersService>,
        Json(request): Json<UpdateUserRequest>,
    ) -> ConduitResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to update user {:?}", user_id);

        let updated_user = users_service.updated_user(user_id, request.user).await?;

        Ok(Json(UserAuthenicationResponse { user: updated_user }))
    }
}
