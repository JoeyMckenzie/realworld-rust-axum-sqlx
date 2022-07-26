use conduit_domain::PingResponse;
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::contexts::authentication_context::AuthenticationProvider;
use crate::{layout::Layout, utilities::http::get};

#[function_component(App)]
pub fn app() -> Html {
    use_effect(|| {
        info!("application started, pinging API");
        spawn_local(async {
            // ping the API to verify it's up and running
            if get::<PingResponse>("/api/ping").await.is_ok() {
                info!("API ping successfully");
            } else {
                error!("API ping returned an error");
            }
        });
        || ()
    });

    html! {
        <body>
            <AuthenticationProvider>
                <Layout />
            </AuthenticationProvider>
        </body>
    }
}
