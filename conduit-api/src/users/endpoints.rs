use axum::{Extension, Json};
use tracing::info;

use conduit_core::errors::ConduitResult;
use conduit_core::users::service::DynUsersService;
use conduit_domain::users::requests::RegisterUserRequest;
use conduit_domain::users::responses::RegisterUserResponse;

use crate::request_validator::RequestValidator;

pub async fn register_user_endpoint(
    RequestValidator(request): RequestValidator<RegisterUserRequest>,
    Extension(users_service): Extension<DynUsersService>,
) -> ConduitResult<Json<RegisterUserResponse>> {
    info!(
        "recieved request to create user {:?}/{:?}",
        request.user.email.as_ref().unwrap(),
        request.user.username.as_ref().unwrap()
    );
    let created_user = users_service.register_user(request.user).await?;
    Ok(Json(RegisterUserResponse { user: created_user }))
}
