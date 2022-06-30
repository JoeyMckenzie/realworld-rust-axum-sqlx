use yew::prelude::*;

use crate::layout::Layout;

#[derive(Clone, Debug, PartialEq)]
pub struct AuthenticationState {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub errors: Vec<String>,
}

impl AuthenticationState {
    pub fn new() -> Self {
        Self {
            username: None,
            email: None,
            password: None,
            errors: vec![],
        }
    }
}

#[function_component(AuthenticationContext)]
pub fn authentication_context() -> Html {
    let authentication_state = use_state(AuthenticationState::new);

    html! {
        <ContextProvider<AuthenticationState> context={(*authentication_state).clone()}>
            <Layout />
        </ContextProvider<AuthenticationState>>
    }
}
