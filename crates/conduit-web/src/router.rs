use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    editor::Editor, home::Home, login::Login, profile::Profile, register::Register, settings::Settings,
};

#[derive(Clone, Routable, PartialEq)]
pub enum ConduitRouter {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/profile/:username")]
    Profile { username: String },
    #[at("/settings")]
    Settings,
    #[at("/article/new")]
    Editor,
    #[not_found]
    #[at("/404")]
    NotFound,
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
        ConduitRouter::Editor => html! {
            <Editor />
        },
        ConduitRouter::NotFound => html! {
            <Home />
        },
    }
}
