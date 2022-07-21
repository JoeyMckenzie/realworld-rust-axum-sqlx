use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, login::Login, register::Register};

#[derive(Clone, Copy, Routable, PartialEq)]
pub enum ConduitRouter {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/profile")]
    Profile,
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
        ConduitRouter::Profile => html! {
            <Register />
        },
    }
}
