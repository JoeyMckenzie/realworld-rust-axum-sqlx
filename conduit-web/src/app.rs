use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{footer::Footer, navbar::Navbar};
use crate::router::{switch, ConduitRouter};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <body>
            <BrowserRouter>
                <Navbar />
                <Switch<ConduitRouter> render={Switch::render(switch)} />
                <Footer />
            </BrowserRouter>
        </body>
    }
}
