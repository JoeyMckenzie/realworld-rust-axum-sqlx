use std::collections::HashMap;

use conduit_domain::users::{
    requests::{LoginUserDto, LoginUserRequest, RegisterUserDto, RegisterUserRequest},
    responses::UserAuthenicationResponse,
};
use lazy_static::lazy_static;
use log::error;
use serde::Deserialize;

use crate::utilities::http::post;

#[derive(Deserialize)]
pub struct Errors {
    pub error: HashMap<String, Vec<String>>,
}

lazy_static! {
    static ref AUTH_ENDPOINT: &'static str = "/api/users";
}

pub async fn register_user(
    username: String,
    email: String,
    password: String,
) -> Result<UserAuthenicationResponse, Vec<String>> {
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

    if let Err(error) = response {
        error!("error while attempting to register user");
        let mapped_errors: Errors = error.into_serde().unwrap();
        let returned_errors: Vec<String> = mapped_errors
            .error
            .into_iter()
            .flat_map(|(_, property_errors)| property_errors)
            .collect();

        return Err(returned_errors);
    }

    Ok(response.unwrap())
}

pub async fn login_user(email: String, password: String) -> Result<UserAuthenicationResponse, Vec<String>> {
    let login_user_request = LoginUserDto {
        email: Some(email),
        password: Some(password),
    };

    let response = post::<UserAuthenicationResponse, LoginUserRequest>(
        &format!("{}/login", *AUTH_ENDPOINT),
        LoginUserRequest {
            user: login_user_request,
        },
    )
    .await;

    if let Err(error) = response {
        error!("error while attempting to login user");
        let mapped_errors: Errors = error.into_serde().unwrap();
        let returned_errors: Vec<String> = mapped_errors
            .error
            .into_iter()
            .flat_map(|(_, property_errors)| property_errors)
            .collect();

        return Err(returned_errors);
    }

    Ok(response.unwrap())
}
