use std::rc::Rc;

use conduit_domain::users::UserDto;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AuthenticationState {
    pub username: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

impl AuthenticationState {
    pub fn new() -> Self {
        Self {
            username: None,
            email: None,
            token: None,
            image: None,
            bio: None,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        // obviously not the best way to do this, will suffice for now
        self.token.is_some() && !self.token.as_ref().unwrap().is_empty()
    }
}

impl Reducible for AuthenticationState {
    type Action = UserDto;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        AuthenticationState {
            username: Some(action.username),
            email: Some(action.email),
            token: Some(action.token),
            image: Some(action.image),
            bio: Some(action.bio),
        }
        .into()
    }
}

type AuthenticationContext = UseReducerHandle<AuthenticationState>;

#[derive(Properties, Debug, PartialEq)]
pub struct AuthenticationProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthenticationProvider)]
pub fn authentication_provider(props: &AuthenticationProviderProps) -> Html {
    let authentication_state = use_reducer(AuthenticationState::new);

    html! {
        <ContextProvider<AuthenticationContext> context={authentication_state}>
            {props.children.clone()}
        </ContextProvider<AuthenticationContext>>
    }
}

pub fn use_authentication_context() -> AuthenticationContext {
    use_context::<AuthenticationContext>().expect("authentication context was not loaded")
}
