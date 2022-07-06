use reqwest::Request;
use yew::prelude::*;

use crate::layout::Layout;

#[function_component(App)]
pub fn app() -> Html {
    // wasm_bindgen_futures::spawn_local(async move {
    //     let client = reqwest::Client::new();
    //     client.get("http://localhost:8080/api/tags").send().await?;
    //     || ()
    // });

    html! {
        <body>
            <Layout/>
        </body>
    }
}
