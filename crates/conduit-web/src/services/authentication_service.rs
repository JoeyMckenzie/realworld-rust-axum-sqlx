use std::collections::HashMap;

use conduit_domain::users::{
    requests::{LoginUserDto, LoginUserRequest, RegisterUserDto, RegisterUserRequest},
    responses::UserAuthenicationResponse,
};
use gloo::console::info;
use lazy_static::lazy_static;
use log::{error, warn};
use serde::Deserialize;

use crate::utilities::{
    errors::ConduitWebResult,
    http::{get, post},
    storage::{get_token, stash_token},
};

pub type AuthenticationResult = Result<UserAuthenicationResponse, Vec<String>>;

#[derive(Deserialize)]
pub struct Errors {
    pub error: HashMap<String, Vec<String>>,
}

lazy_static! {
    static ref AUTH_ENDPOINT: &'static str = "/api/users";
    static ref USER_ENDPOINT: &'static str = "/api/user";
}

pub async fn register_user(username: String, email: String, password: String) -> AuthenticationResult {
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

    let user = response.unwrap();

    if stash_token(user.user.token.clone()).is_err() {
        error!("could not stash the user token");
    }

    Ok(user)
}

pub async fn login_user(email: String, password: String) -> AuthenticationResult {
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

    let user = response.unwrap();

    if stash_token(user.user.token.clone()).is_err() {
        error!("could not stash the user token");
    }

    Ok(user)
}

pub async fn get_current_user() -> ConduitWebResult<()> {
    if let Ok(stashed_token) = get_token() {
        info!("retrieving stashed user");
        let response = get::<UserAuthenicationResponse>(*USER_ENDPOINT).await;
    } else {
        warn!("no user token in storage");
    }

    Ok(())
}
