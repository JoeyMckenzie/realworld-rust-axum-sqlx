use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::{
    components::{footer::Footer, navbar::Navbar},
    router::{router_map, ConduitRouter},
};

#[function_component(Layout)]
pub fn layout() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <Switch<ConduitRouter> render={Switch::render(router_map)} />
            <Footer />
        </BrowserRouter>
    }
}
