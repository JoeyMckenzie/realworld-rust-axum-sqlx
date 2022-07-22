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
            let ping_response = get::<PingResponse>("/api/ping").await;

            if let Ok(value) = ping_response {
                info!("API ping successfully, response: {:?}", value);
            } else {
                error!("API ping returned an error");
            }

            // load in user information
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
