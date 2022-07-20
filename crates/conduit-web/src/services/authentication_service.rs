use conduit_domain::users::{
    requests::{RegisterUserDto, RegisterUserRequest},
    responses::UserAuthenicationResponse,
};
use lazy_static::lazy_static;
use log::error;
use wasm_bindgen::JsValue;

use crate::utilities::http::post;

lazy_static! {
    static ref AUTH_ENDPOINT: &'static str = "/api/users";
}

pub async fn register_user(
    username: String,
    email: String,
    password: String,
) -> Result<UserAuthenicationResponse, JsValue> {
    let register_user_request = RegisterUserDto {
        username: Some(username),
        email: Some(email),
        password: Some(password),
    };

    let response = post::<UserAuthenicationResponse, RegisterUserRequest>(
        *AUTH_ENDPOINT,
        RegisterUserRequest {
            user: register_user_request,
        },
    )
    .await;

    if response.is_err() {
        error!("error while attempting to register user");
    }

    Ok(response.unwrap())
}
