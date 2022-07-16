use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::active_link::ActiveLink;
use crate::router::ConduitRouter;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <Link<ConduitRouter> classes="navbar-brand" to={ConduitRouter::Home}>{ "conduit" }</Link<ConduitRouter>>
                <ul class="nav navbar-nav pull-xs-right">
                    <li class="nav-item">
                        <ActiveLink to={ConduitRouter::Home} display_as="Home" />
                    </li>
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
                        <ActiveLink to={ConduitRouter::Login} display_as="Sign in" />
                    </li>
                    <li class="nav-item">
                        <ActiveLink to={ConduitRouter::Register} display_as="Sign up" />
                    </li>
                </ul>
            </div>
        </nav>
    }
}
