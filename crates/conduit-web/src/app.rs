use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{layout::Layout, utilities::http::http_request_json};

#[function_component(App)]
pub fn app() -> Html {
    use_effect(|| {
        info!("application started, pinging API");
        spawn_local(async {
            let ping_response = http_request_json("/api/ping", "GET", None).await;

            if ping_response.is_ok() {
                info!("API ping successfully");
            } else {
                error!("API ping returned an error");
            }
        });
        || ()
    });

    html! {
        <body>
            <Layout/>
        </body>
    }
}
