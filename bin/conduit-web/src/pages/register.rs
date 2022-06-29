use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use conduit_domain::users::requests::RegisterUserDto;

use crate::router::ConduitRouter;

#[function_component(Register)]
pub fn register() -> Html {
    let user_registration_form = use_state(RegisterUserDto::default);

    let onsubmit = {
        let user_register = user_registration_form.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
        })
    };

    let on_username_input = {
        let current_user_registration_form = user_registration_form.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            log!("username {}", input.value());

            let mut current_form = (*current_user_registration_form).clone();
            current_form.username = Some(input.value());
            current_user_registration_form.set(current_form);
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign in" }</h1>
                        <p class="text-xs-center">
                            <Link<ConduitRouter> to={ConduitRouter::Login}>{ "Have an account?" }</Link<ConduitRouter>>
                        </p>

                        <ul class="error-messages">
                            <li>{ "That email is already taken" }</li>
                        </ul>

                        <form {onsubmit}>
                            <fieldset class="form-group">
                                <input class="form-control form-control-lg" type="text" placeholder="Your Name"/>
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Email"
                                    value={user_registration_form.username.clone()}
                                    oninput={on_username_input}
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input class="form-control form-control-lg" type="password" placeholder="Password"/>
                            </fieldset>
                            <button class="btn btn-lg btn-primary pull-xs-right">
                                { "Sign up" }
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
