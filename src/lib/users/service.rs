use crate::lib::users::domain::responses::RegisterUserResponse;
use axum::http::StatusCode;

pub async fn create_user() -> Result<RegisterUserResponse, StatusCode> {
    Ok(RegisterUserResponse {})
}
