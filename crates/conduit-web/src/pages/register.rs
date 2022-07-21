use log::info;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::authentication_error_list::AuthenticationErrorList;
use crate::router::ConduitRouter;
use crate::services::authentication_service::register_user;

#[function_component(Register)]
pub fn register() -> Html {
    let history = use_history().expect("error while loading location");
    let username = use_state(String::default);
    let email = use_state(String::default);
    let password = use_state(String::default);
    let register_errors = use_state(Vec::<String>::new);

    let onsubmit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let register_errors = register_errors.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            let username = username.clone();
            let email = email.clone();
            let password = password.clone();
            let history = history.clone();
            let register_errors = register_errors.clone();

            spawn_local(async move {
                let register_response = register_user((*username).clone(), (*email).clone(), (*password).clone()).await;

                if let Err(returned_errors) = register_response {
                    register_errors.set(returned_errors);
                } else {
                    history.push(ConduitRouter::Home);
                }
            });
        })
    };

    let email_oninput = {
        let email = email.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
            info!("email {:?}", email);
        })
    };

    let username_oninput = {
        let username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
            info!("username {:?}", username);
        })
    };

    let password_oninput = {
        let password = password.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
            info!("password {:?}", password);
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

                        <AuthenticationErrorList errors={(*register_errors).clone()} />

                        <form {onsubmit}>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Your Name"
                                    value={(*username).clone()}
                                    oninput={username_oninput}
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Email"
                                    value={(*email).clone()}
                                    oninput={email_oninput}
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="password"
                                    placeholder="Password"
                                    value={(*password).clone()}
                                    oninput={password_oninput}
                                />
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
