use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, login::Login};

#[derive(Clone, Routable, PartialEq)]
pub enum ConduitRouter {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
}

pub fn switch(routes: &ConduitRouter) -> Html {
    match routes {
        ConduitRouter::Home => html! {
            <Home />
        },
        ConduitRouter::Login => html! {
            <Login />
        },
    }
}
