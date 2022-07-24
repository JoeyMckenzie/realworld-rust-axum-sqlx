use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, login::Login, profile::Profile, register::Register, settings::Settings};

#[derive(Clone, Routable, PartialEq)]
pub enum ConduitRouter {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/profile:/:username")]
    Profile { username: String },
    #[at("/settings")]
    Settings,
}

pub fn router_map(routes: &ConduitRouter) -> Html {
    match routes {
        ConduitRouter::Home => html! {
            <Home />
        },
        ConduitRouter::Login => html! {
            <Login />
        },
        ConduitRouter::Register => html! {
            <Register />
        },
        ConduitRouter::Profile { username } => html! {
            <Profile username={username.clone()} />
        },
        ConduitRouter::Settings => html! {
            <Settings />
        },
    }
}
