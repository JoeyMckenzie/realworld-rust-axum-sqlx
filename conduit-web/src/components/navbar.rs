use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::ConduitRouter;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <a class="navbar-brand" href="index.html">{ "conduit" }</a>
                <ul class="nav navbar-nav pull-xs-right">
                    <li class="nav-item">
                        // <!-- Add "active" class when you're on that page" -->
                        <Link<ConduitRouter> classes="nav-link active" to={ConduitRouter::Home}>{ "Home" }</Link<ConduitRouter>>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="">
                            <i class="ion-compose"></i>{ "New Article" }
                        </a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="">
                            <i class="ion-gear-a"></i>{ "Settings" }
                        </a>
                    </li>
                    <li class="nav-item">
                        <Link<ConduitRouter> classes="nav-link" to={ConduitRouter::Login}>{ "Sign in" }</Link<ConduitRouter>>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="">{ "Sign up" }</a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
