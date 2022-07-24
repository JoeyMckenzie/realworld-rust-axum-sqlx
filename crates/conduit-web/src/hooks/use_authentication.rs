use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::authentication_context::use_authentication_context;
use crate::router::ConduitRouter;
use crate::services::authentication_service::{login_user, register_user, AuthenticationResult};

#[derive(Debug)]
pub struct UseAuthenticationHook {
    pub username: String,
    pub username_oninput: Callback<InputEvent>,
    pub email: String,
    pub email_oninput: Callback<InputEvent>,
    pub password: String,
    pub password_oninput: Callback<InputEvent>,
    pub onsubmit: Callback<FocusEvent>,
    pub errors: Vec<String>,
}

pub fn use_authentication(include_username: bool) -> UseAuthenticationHook {
    let history = use_history().expect("error while loading location");
    let username = use_state(String::default);
    let email = use_state(String::default);
    let password = use_state(String::default);
    let errors = use_state(Vec::<String>::new);
    let authentication_context = use_authentication_context();

    let onsubmit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let errors = errors.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            let username = username.clone();
            let email = email.clone();
            let password = password.clone();
            let history = history.clone();
            let errors = errors.clone();
            let authentication_context = authentication_context.clone();

            spawn_local(async move {
                let authentication_response: AuthenticationResult = if include_username {
                    register_user((*username).clone(), (*email).clone(), (*password).clone()).await
                } else {
                    login_user((*email).clone(), (*password).clone()).await
                };

                if let Err(returned_errors) = authentication_response {
                    errors.set(returned_errors);
                } else {
                    let user = authentication_response.unwrap().user;
                    authentication_context.dispatch(user);
                    history.push(ConduitRouter::Home);
                }
            });
        })
    };

    let email_oninput = {
        let email = email.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let username_oninput = {
        let username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let password_oninput = {
        let password = password.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    UseAuthenticationHook {
        username: (*username).clone(),
        username_oninput,
        email: (*email).clone(),
        email_oninput,
        password: (*password).clone(),
        password_oninput,
        onsubmit,
        errors: (*errors).clone(),
    }
}
