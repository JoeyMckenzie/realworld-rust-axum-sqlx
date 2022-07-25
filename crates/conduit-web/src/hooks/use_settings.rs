use conduit_domain::users::{
    requests::{UpdateUserDto, UpdateUserRequest},
    responses::UserAuthenicationResponse,
};
use gloo::console::error;
use log::info;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlInputElement, HtmlTextAreaElement, InputEvent};
use yew::prelude::*;

use crate::{contexts::authentication_context::use_authentication_context, utilities::http::put};

#[derive(Debug)]
pub struct UseSettingsHook {
    pub image: String,
    pub image_oninput: Callback<InputEvent>,
    pub username: String,
    pub username_oninput: Callback<InputEvent>,
    pub bio: String,
    pub bio_oninput: Callback<InputEvent>,
    pub email: String,
    pub email_oninput: Callback<InputEvent>,
    pub password: String,
    pub password_oninput: Callback<InputEvent>,
    pub onsubmit: Callback<FocusEvent>,
}

pub fn use_settings() -> UseSettingsHook {
    let authentication_context = use_authentication_context();

    let image = use_state(String::default);
    let username = use_state(String::default);
    let bio = use_state(String::default);
    let email = use_state(String::default);
    let password = use_state(String::default);

    {
        let authentication_context = use_authentication_context();
        let image = image.clone();
        let username = username.clone();
        let bio = bio.clone();
        let email = email.clone();

        use_effect_with_deps(
            move |context| {
                if context.is_authenticated() {
                    image.set(context.image.as_ref().unwrap().to_owned());
                    username.set(context.username.as_ref().unwrap().to_owned());
                    bio.set(context.bio.as_ref().unwrap().to_owned());
                    email.set(context.email.as_ref().unwrap().to_owned());
                }
                || ()
            },
            authentication_context,
        )
    }

    let image_oninput = {
        let image = image.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            image.set(input.value());
        })
    };

    let username_oninput = {
        let username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let bio_oninput = {
        let bio = bio.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            bio.set(input.value());
        })
    };

    let email_oninput = {
        let email = email.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let password_oninput = {
        let password = password.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let onsubmit = {
        let image = image.clone();
        let username = username.clone();
        let bio = bio.clone();
        let email = email.clone();
        let password = password.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            let authentication_context = authentication_context.clone();
            let image = image.clone();
            let username = username.clone();
            let bio = bio.clone();
            let email = email.clone();
            let password = password.clone();

            spawn_local(async move {
                let update_request = UpdateUserDto {
                    email: Some((*email).clone()),
                    username: Some((*username).clone()),
                    password: Some((*password).clone()),
                    bio: Some((*bio).clone()),
                    image: Some((*image).clone()),
                };

                let response = put::<UserAuthenicationResponse, UpdateUserRequest>(
                    "/api/user",
                    UpdateUserRequest { user: update_request },
                )
                .await;

                if let Ok(user_response) = response {
                    info!("user successfully updated");
                    image.set(user_response.user.image.clone());
                    username.set(user_response.user.username.clone());
                    bio.set(user_response.user.bio.clone());
                    email.set(user_response.user.email.clone());
                    password.set(String::default());

                    // set the updated user information in context
                    authentication_context.dispatch(user_response.user);
                } else {
                    error!("error while attempting to update user");
                }
            });
        })
    };

    UseSettingsHook {
        image: (*image).clone(),
        image_oninput,
        username: (*username).clone(),
        username_oninput,
        bio: (*bio).clone(),
        bio_oninput,
        email: (*email).clone(),
        email_oninput,
        password: (*password).clone(),
        password_oninput,
        onsubmit,
    }
}
