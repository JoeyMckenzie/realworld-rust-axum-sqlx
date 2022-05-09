use crate::errors::ConduitError;
use crate::users::domain::requests::RegisterUserRequest;
use crate::users::domain::responses::{RegisterUserResponse, UserDto};
use axum::{Extension, Json};
use conduit_shared::context::SyncAppContext;
use tracing::{error, info};
use validator::Validate;

pub async fn register_user(
    Json(request): Json<RegisterUserRequest>,
    Extension(context): Extension<SyncAppContext>,
) -> Result<Json<RegisterUserResponse>, ConduitError> {
    match request.validate() {
        Ok(_) => info!("nice"),
        Err(e) => error!("{:?}", e),
    }

    let user_dto = request.user.unwrap();

    let user = UserDto {
        email: user_dto.email.unwrap(),
        username: user_dto.username.unwrap(),
    };

    Ok(Json(RegisterUserResponse { user }))
}
