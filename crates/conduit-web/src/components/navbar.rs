use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::active_link::ActiveLink;
use crate::contexts::authentication_context::use_authentication_context;
use crate::router::ConduitRouter;
use crate::services::authentication_service::get_current_user;
use crate::utilities::storage::clear_token;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let authentication_context = use_authentication_context();

    {
        let authentication_context = authentication_context.clone();

        use_effect(move || {
            spawn_local(async {
                let current_user = get_current_user().await;
            });
            || ()
        });
    }

    let maybe_authentication_links = move || -> Html {
        let authentication_context = authentication_context.clone();
        let clear_token_onclick = Callback::from(|_| {
            clear_token();
        });

        if authentication_context.is_authenticated() {
            let email = authentication_context.email.as_ref().unwrap();

            html! {
                <>
                    <li class="nav-item">
                        <a class="nav-link" href="">
                            <i class="ion-compose"></i>{ " \u{00a0}New Article" }
                        </a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="">
                            <i class="ion-gear-a"></i>{ " \u{00a0}Settings" }
                        </a>
                    </li>
                    <li class="nav-item">
                        <ActiveLink to={ConduitRouter::Profile} display_as={email.clone()} />
                    </li>
                    <li onclick={clear_token_onclick} class="nav-item">
                        <span class="nav-link">
                            { "Clear token" }
                        </span>
                    </li>
                </>
            }
        } else {
            html! {
                <>
                    <li class="nav-item">
                        <ActiveLink to={ConduitRouter::Login} display_as="Sign in" />
                    </li>
                    <li class="nav-item">
                        <ActiveLink to={ConduitRouter::Register} display_as="Sign up" />
                    </li>
                </>
            }
        }
    };

    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <Link<ConduitRouter> classes="navbar-brand" to={ConduitRouter::Home}>{ "conduit" }</Link<ConduitRouter>>
                <ul class="nav navbar-nav pull-xs-right">
                    <li class="nav-item">
                        <ActiveLink to={ConduitRouter::Home} display_as="Home" />
                    </li>
                    {maybe_authentication_links()}
                </ul>
            </div>
        </nav>
    }
}
