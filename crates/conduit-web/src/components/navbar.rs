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
            spawn_local(async move {
                let current_user = get_current_user().await;
                if let Ok(user_meta) = current_user {
                    authentication_context.dispatch(user_meta.user);
                }
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
            let username = authentication_context.username.as_ref().unwrap();

            html! {
                <>
                    <li class="nav-item">
                        <ActiveLink to={ConduitRouter::Editor}>
                            <i class="ion-compose"></i>{ " \u{00a0}New Article" }
                        </ActiveLink>
                    </li>
                    <li class="nav-item">
                        <ActiveLink to={ConduitRouter::Settings}>
                            <i class="ion-gear-a"></i>{ " \u{00a0}Settings" }
                        </ActiveLink>
                    </li>
                    <li class="nav-item">
                        <ActiveLink to={ConduitRouter::Profile { username: username.clone() }}>
                            { username.clone() }
                        </ActiveLink>
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
                        <ActiveLink to={ConduitRouter::Login}>
                            { "Sign in" }
                        </ActiveLink>
                    </li>
                    <li class="nav-item">
                        <ActiveLink to={ConduitRouter::Register}>
                            { "Sign up" }
                        </ActiveLink>
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
                        <ActiveLink to={ConduitRouter::Home}>
                            { "Home" }
                        </ActiveLink>
                    </li>
                    {maybe_authentication_links()}
                </ul>
            </div>
        </nav>
    }
}
