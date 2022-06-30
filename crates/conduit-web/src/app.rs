use yew::prelude::*;

use crate::layout::Layout;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <body>
            <Layout/>
        </body>
    }
}
