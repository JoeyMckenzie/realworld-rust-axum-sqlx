use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::ConduitRouter;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <Link<ConduitRouter> classes="navbar-brand" to={ConduitRouter::Home}>{ "conduit" }</Link<ConduitRouter>>
                <ul class="nav navbar-nav pull-xs-right">
                    <li class="nav-item">
                        // <!-- Add "active" class when you're on that page" -->
                        <Link<ConduitRouter> classes="nav-link active" to={ConduitRouter::Home}>{ "Home" }</Link<ConduitRouter>>
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
                        <Link<ConduitRouter> classes="nav-link" to={ConduitRouter::Login}>{ "Sign in" }</Link<ConduitRouter>>
                    </li>
                    <li class="nav-item">
                        <Link<ConduitRouter> classes="nav-link" to={ConduitRouter::Register}>{ "Sign up" }</Link<ConduitRouter>>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
