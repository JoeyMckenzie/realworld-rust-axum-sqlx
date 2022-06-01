use crate::errors::ConduitEndpointResult;
use axum::{Extension, Json};
use conduit_core::users::service::DynUserService;
use conduit_domain::users::models::UserDto;
use conduit_domain::users::requests::RegisterUserRequest;
use conduit_domain::users::responses::RegisterUserResponse;

pub async fn register_user_endpoint(
    Json(request): Json<RegisterUserRequest>,
    Extension(users_service): Extension<DynUserService>,
) -> ConduitEndpointResult<Json<RegisterUserResponse>> {
    let user_dto = request.user.unwrap();

    let _test = users_service.register_user(user_dto).await;

    let user = UserDto {
        email: String::from("email"),
        username: String::from("username"),
        bio: String::from("bio"),
        image: String::from("image"),
        token: String::from("token"),
    };

    Ok(Json(RegisterUserResponse { user }))
}
