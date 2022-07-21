use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::active_link::ActiveLink;
use crate::contexts::authentication_context::use_authentication_context;
use crate::router::ConduitRouter;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let authentication_context = use_authentication_context();

    let maybe_authentication_links = move || -> Html {
        let authentication_context = authentication_context.clone();

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
