use crate::errors::ConduitEndpointResult;
use crate::request_validator::RequestValidator;
use axum::{Extension, Json};
use conduit_core::users::service::DynUsersService;
use conduit_domain::users::models::UserDto;
use conduit_domain::users::requests::RegisterUserRequest;
use conduit_domain::users::responses::RegisterUserResponse;

pub async fn register_user_endpoint(
    RequestValidator(request): RequestValidator<RegisterUserRequest>,
    Extension(users_service): Extension<DynUsersService>,
) -> ConduitEndpointResult<Json<RegisterUserResponse>> {
    let register_response = users_service.register_user(request.user).await?;

    let user = UserDto {
        email: String::from("email"),
        username: String::from("username"),
        bio: String::from("bio"),
        image: String::from("image"),
        token: String::from("token"),
    };

    Ok(Json(RegisterUserResponse { user }))
}
